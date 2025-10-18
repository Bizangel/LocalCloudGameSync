use crate::{
    commands::{
        CheckSyncResult, check_sync_command, pull_command_with_update_callback,
        push_command_with_update_callback,
    },
    common::Revision,
    config::RuntimeSyncConfig,
    ui::common::{UIEvent, UserChoice, WebViewCommand, WebViewState},
};
use std::sync::mpsc::Receiver;
use tao::event_loop::EventLoopProxy;

fn block_until<T, F>(receiver: &Receiver<T>, predicate: F) -> T
where
    F: Fn(&T) -> bool,
{
    loop {
        match receiver.recv() {
            Ok(msg) => {
                if predicate(&msg) {
                    return msg;
                }
            }
            Err(e) => panic!("{e}"), // channel disconnected
        }
    }
}

#[derive(Debug, Clone)]
pub enum SyncThreadCommand {
    UIReady,
    UserChoice { choice: UserChoice },
}

fn send_ui_display_update(
    ui_proxy: &EventLoopProxy<UIEvent>,
    titletext: impl Into<String>,
    subtext: impl Into<String>,
) {
    let cmd = UIEvent::WebViewCommand {
        command: WebViewCommand::WebViewUpdate {
            title_text: titletext.into(),
            sub_text: subtext.into(),
        },
    };
    let _ = ui_proxy.send_event(cmd);
}

fn send_ui_change_state(ui_proxy: &EventLoopProxy<UIEvent>, state: WebViewState) {
    let _ = ui_proxy.send_event(UIEvent::WebViewCommand {
        command: WebViewCommand::WebViewStateChange { state: state },
    });
}

fn push_to_remote(
    sync_config: &RuntimeSyncConfig,
    ui_proxy: &EventLoopProxy<UIEvent>,
    remote_head: &Option<Revision>,
    main_sync_title: &str,
) -> Result<(), String> {
    send_ui_display_update(
        &ui_proxy,
        main_sync_title,
        "Local changes found - saving to remote...",
    );

    let push_title = format!("Uploading {} save files", sync_config.remote_sync_key);
    push_command_with_update_callback(
        sync_config,
        remote_head.as_ref().map(|head| head.hash.as_str()),
        |txt| {
            send_ui_display_update(&ui_proxy, &push_title, &txt);
        },
    )?;

    send_ui_display_update(
        &ui_proxy,
        String::from("Sync Success!"),
        "Uploaded to remote!",
    );

    Ok(())
}

fn pull_from_remote(
    sync_config: &RuntimeSyncConfig,
    ui_proxy: &EventLoopProxy<UIEvent>,
    remote_head: &Option<Revision>,
    main_sync_title: &str,
) -> Result<(), String> {
    send_ui_display_update(
        &ui_proxy,
        main_sync_title,
        "Newer version on remote found! Downloading from remote...",
    );

    let pull_title = format!("Downloading {} save files", sync_config.remote_sync_key);
    pull_command_with_update_callback(
        sync_config,
        remote_head.as_ref().map(|head| head.hash.as_str()),
        |txt| {
            send_ui_display_update(&ui_proxy, &pull_title, &txt);
        },
    )?;

    send_ui_display_update(
        &ui_proxy,
        String::from("Sync Success!"),
        "Downloaded from remote!",
    );

    Ok(())
}

pub fn do_sync(
    sync_config: &RuntimeSyncConfig,
    ui_proxy: &EventLoopProxy<UIEvent>,
    sync_rx: &Receiver<SyncThreadCommand>,
) -> Result<bool, String> {
    send_ui_change_state(ui_proxy, WebViewState::Loading);
    let main_sync_title = format!("Syncing {}", sync_config.remote_sync_key);
    send_ui_display_update(&ui_proxy, &main_sync_title, "Checking remote...");

    let (check_sync_result, remote_head) = check_sync_command(&sync_config)?;

    match check_sync_result {
        // === Success Up To Date Logic
        CheckSyncResult::UpToDate => {
            send_ui_display_update(
                &ui_proxy,
                String::from("Sync Success!"),
                "Local is up to date!",
            );
            return Ok(true);
        }
        // === Pulling from remote logic ===
        CheckSyncResult::FastForwardLocal => {
            push_to_remote(sync_config, ui_proxy, &remote_head, &main_sync_title)?;
            return Ok(true);
        }
        // === Pushing to remote cleanup ===
        CheckSyncResult::FastForwardRemote => {
            push_to_remote(sync_config, ui_proxy, &remote_head, &main_sync_title)?;
            return Ok(true);
        }
        CheckSyncResult::RemoteEmpty => {
            send_ui_change_state(&ui_proxy, WebViewState::RemoteEmpty);
            send_ui_display_update(
                &ui_proxy,
                String::from("Empty Remote Confirmation"),
                format!(
                    "Remote for key {} empty! Do you wish to push to initialize remote?",
                    &sync_config.remote_sync_key
                ),
            );

            loop {
                match sync_rx.recv() {
                    Ok(SyncThreadCommand::UserChoice {
                        choice: UserChoice::Push,
                    }) => {
                        break; // do same as push above
                    }
                    Ok(SyncThreadCommand::UserChoice {
                        choice: UserChoice::Close,
                    }) => {
                        return Ok(false); // user chose to stop
                    }
                    Err(e) => panic!("{e}"),
                    _ => continue,
                }
            }

            send_ui_change_state(&ui_proxy, WebViewState::Loading);
            push_to_remote(sync_config, ui_proxy, &remote_head, &main_sync_title)?;

            return Ok(true);
        }
        CheckSyncResult::Conflict { local, remote } => {
            send_ui_change_state(&ui_proxy, WebViewState::Conflict);
            send_ui_display_update(&ui_proxy, &main_sync_title, format!("Conflict found!"));

            let selected_choice: UserChoice;
            loop {
                match sync_rx.recv() {
                    Ok(SyncThreadCommand::UserChoice { choice }) => {
                        match choice {
                            UserChoice::Pull | UserChoice::Push => {
                                selected_choice = choice;
                                break;
                            }
                            UserChoice::Close => return Ok(false), // user is closing stop
                            _ => continue,                         // ignore any other
                        }
                    }
                    Err(e) => panic!("{e}"),
                    _ => continue,
                }
            }

            // TODO: Send info about merge choice
            match selected_choice {
                UserChoice::Pull => {
                    pull_from_remote(sync_config, ui_proxy, &remote_head, &main_sync_title)?;
                }
                UserChoice::Push => {
                    push_to_remote(sync_config, ui_proxy, &remote_head, &main_sync_title)?;
                }
                _ => return Ok(false),
            }

            return Ok(true);
        }
    }
}

pub fn sync_thread_main(
    sync_config: RuntimeSyncConfig,
    ui_proxy: EventLoopProxy<UIEvent>,
    sync_rx: Receiver<SyncThreadCommand>,
) {
    // Await for UI.
    block_until(&sync_rx, |cmd| matches!(cmd, SyncThreadCommand::UIReady));
    loop {
        match do_sync(&sync_config, &ui_proxy, &sync_rx) {
            Ok(success) => {
                if !success {
                    let _ = ui_proxy.send_event(UIEvent::SyncFailedEvent);
                    break; // done
                }

                // We're done - let UI thread exit with success
                // wait 1 second so user can read - it gives nice feeling maybe remove it later?
                send_ui_change_state(&ui_proxy, WebViewState::Success);
                std::thread::sleep(std::time::Duration::from_secs(1));
                let _ = ui_proxy.send_event(UIEvent::SyncSuccessCompletedEvent);
                break; // done
            }
            Err(e) => {
                send_ui_change_state(&ui_proxy, WebViewState::Error);
                send_ui_display_update(&ui_proxy, "Sync Error", format!("{}", e));

                // Await until user resolves error conflict (Either retry, close or continue)
                let selected_choice: UserChoice;
                loop {
                    match sync_rx.recv() {
                        Ok(SyncThreadCommand::UserChoice { choice }) => {
                            match choice {
                                UserChoice::Close
                                | UserChoice::ContinueOffline
                                | UserChoice::Retry => {
                                    selected_choice = choice;
                                    break;
                                }
                                _ => continue, // ignore any other
                            }
                        }
                        Err(e) => panic!("{e}"),
                        _ => continue,
                    }
                }

                match selected_choice {
                    UserChoice::Close => {
                        // Exit with 1
                        let _ = ui_proxy.send_event(UIEvent::SyncFailedEvent);
                        break; // done
                    }
                    UserChoice::Retry => continue, // re-do outer loop
                    UserChoice::ContinueOffline => {
                        // Send as if sync was successful. This will exit with code 0 allowing game to launch.
                        let _ = ui_proxy.send_event(UIEvent::SyncSuccessCompletedEvent);
                        break; // done
                    }
                    _ => continue, // unexpected choice - just continue
                }
            }
        }
    }
}
