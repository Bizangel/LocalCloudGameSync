use crate::{
    commands::{CheckSyncResult, check_sync_command},
    config::RuntimeSyncConfig,
    ui::common::{UIEvent, UserChoice, WebViewState},
};
use std::sync::mpsc::Receiver;
use tao::event_loop::EventLoopProxy;

mod interactions;
mod operations;
mod ui_messages;

use interactions::{
    ErrorResolution, SyncOutcome, handle_conflict, handle_remote_empty, handle_sync_error,
};
use operations::{pull_from_remote, push_to_remote};
use ui_messages::{show_loading_step, show_success_message};

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

fn do_sync(
    sync_config: &RuntimeSyncConfig,
    ui_proxy: &EventLoopProxy<UIEvent>,
    sync_rx: &Receiver<SyncThreadCommand>,
) -> Result<SyncOutcome, String> {
    let main_sync_title = format!("Syncing {}", sync_config.game_display_name);
    show_loading_step(ui_proxy, &main_sync_title, "Checking remote...");

    let (check_sync_result, remote_head) = check_sync_command(sync_config)?;

    match check_sync_result {
        CheckSyncResult::UpToDate => {
            show_success_message(
                ui_proxy,
                &sync_config.game_display_name,
                "Local is up to date!",
            );
            Ok(SyncOutcome::Completed)
        }
        CheckSyncResult::FastForwardLocal => {
            pull_from_remote(sync_config, ui_proxy, &remote_head, &main_sync_title)?;
            Ok(SyncOutcome::Completed)
        }
        CheckSyncResult::FastForwardRemote => {
            push_to_remote(sync_config, ui_proxy, &remote_head, &main_sync_title)?;
            Ok(SyncOutcome::Completed)
        }
        CheckSyncResult::RemoteEmpty => handle_remote_empty(
            sync_config,
            ui_proxy,
            sync_rx,
            &remote_head,
            &main_sync_title,
        ),
        CheckSyncResult::Conflict { local, remote } => handle_conflict(
            sync_config,
            ui_proxy,
            sync_rx,
            &remote_head,
            &main_sync_title,
            &local,
            &remote,
        ),
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
            Ok(SyncOutcome::Completed) => {
                // We're done - let UI thread exit with success
                ui_messages::send_ui_change_state(&ui_proxy, WebViewState::Success);
                std::thread::sleep(std::time::Duration::from_secs(1));
                let _ = ui_proxy.send_event(UIEvent::SyncSuccessCompletedEvent);
                break;
            }
            Ok(SyncOutcome::Cancelled) => {
                let _ = ui_proxy.send_event(UIEvent::SyncFailedEvent);
                break;
            }
            Err(e) => {
                match handle_sync_error(&ui_proxy, &sync_rx, &sync_config.game_display_name, &e) {
                    ErrorResolution::Retry => continue,
                    ErrorResolution::Close => {
                        let _ = ui_proxy.send_event(UIEvent::SyncFailedEvent);
                        break;
                    }
                    ErrorResolution::ContinueOffline => {
                        let _ = ui_proxy.send_event(UIEvent::SyncSuccessCompletedEvent);
                        break;
                    }
                }
            }
        }
    }
}
