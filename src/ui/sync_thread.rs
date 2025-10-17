use crate::{
    commands::{CheckSyncResult, check_sync_command, pull_command_with_update_callback},
    config::RuntimeSyncConfig,
    ui::common::{ResolveConflictChoice, ResolveErrorChoice, UIEvent, WebViewState},
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
    ResolveConflict { choice: ResolveConflictChoice },
    ResolveError { choice: ResolveErrorChoice },
}

fn send_ui_display_update(
    ui_proxy: &EventLoopProxy<UIEvent>,
    titletext: impl Into<String>,
    subtext: impl Into<String>,
) {
    let req = UIEvent::WebViewUpdateRequest {
        title_text: titletext.into(),
        sub_text: subtext.into(),
    };
    let _ = ui_proxy.send_event(req);
}

fn send_ui_change_state(ui_proxy: &EventLoopProxy<UIEvent>, state: WebViewState) {
    let _ = ui_proxy.send_event(UIEvent::WebViewStateChangeRequest { state });
}

pub fn do_sync(
    sync_config: &RuntimeSyncConfig,
    ui_proxy: &EventLoopProxy<UIEvent>,
    _sync_rx: &Receiver<SyncThreadCommand>,
) -> Result<(), String> {
    send_ui_change_state(ui_proxy, WebViewState::Loading);
    let main_sync = format!("Syncing {}", sync_config.remote_sync_key);
    send_ui_display_update(&ui_proxy, &main_sync, "Checking remote...");

    let (check_sync_result, remote_head) = check_sync_command(&sync_config, false)?;

    match check_sync_result {
        CheckSyncResult::UpToDate => {
            send_ui_display_update(&ui_proxy, &main_sync, "Local is up to date!");
            return Ok(());
        }
        CheckSyncResult::FastForwardLocal => {
            send_ui_display_update(
                &ui_proxy,
                &main_sync,
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

            return Ok(());
        }
        CheckSyncResult::FastForwardRemote => {
            send_ui_display_update(
                &ui_proxy,
                &main_sync,
                "Local changes found - saving to remote...",
            );
            // TODO: take action
            return Ok(());
        }
        CheckSyncResult::RemoteEmpty => {
            send_ui_display_update(
                &ui_proxy,
                &main_sync,
                format!(
                    "Remote for key {} empty! Do you wish to push to initialize remote!",
                    &sync_config.remote_sync_key
                ),
            );

            // TODO: take action

            return Ok(());
        }
        CheckSyncResult::Conflict => {
            send_ui_display_update(
                &ui_proxy,
                &main_sync,
                format!("Conflict found for key! Found: "),
            );

            // await until resolve
            // let x = block_until(&sync_rx, |cmd| {
            //     matches!(cmd, SyncThreadCommand::ResolveConflict { choice: _ })
            // });

            // then continue
            // TODO: take action

            return Ok(());
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
            Ok(_) => {
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
                let x = block_until(&sync_rx, |cmd| {
                    matches!(cmd, SyncThreadCommand::ResolveError { choice: _ })
                });
                let SyncThreadCommand::ResolveError { choice } = x else {
                    unreachable!("block_until guarantees this is ResolveError")
                };

                match choice {
                    ResolveErrorChoice::Close => {
                        // Exit with 1
                        let _ = ui_proxy.send_event(UIEvent::SyncFailedEvent);
                        break; // done
                    }
                    ResolveErrorChoice::Retry => continue, // re-do outer loop
                    ResolveErrorChoice::ContinueOffline => {
                        // Send as if sync was successful. This will exit with code 0 allowing game to launch.
                        let _ = ui_proxy.send_event(UIEvent::SyncSuccessCompletedEvent);
                        break; // done
                    }
                }
            }
        }
    }
}
