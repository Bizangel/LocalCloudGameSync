use crate::{
    commands::{CheckSyncResult, check_sync_command},
    config::config_commons::load_config,
    ui::common::{SyncThreadCommand, SyncThreadContext, UIEvent, WebViewState},
};
use std::sync::mpsc::Receiver;

mod interactions;
mod operations;
mod ui_messages;

use interactions::{
    ErrorResolution, SyncOutcome, handle_conflict, handle_remote_empty, handle_sync_error,
};
use operations::{pull_from_remote, push_to_remote};

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

fn do_sync(sync_key: &str, context: &mut SyncThreadContext) -> Result<SyncOutcome, String> {
    // Load config
    let sync_config = load_config(sync_key, context.config_file_override.as_deref())?;
    context.game_display_name = Some(sync_config.game_display_name.clone());
    let context: &SyncThreadContext = context; // drop mutability

    let main_sync_title = format!("Syncing {}", sync_config.game_display_name);
    context.show_loading_step(&main_sync_title, "Checking remote...");

    let (check_sync_result, remote_head) = check_sync_command(&sync_config)?;

    match check_sync_result {
        CheckSyncResult::UpToDate => {
            context.show_success_message(&sync_config.game_display_name, "Local is up to date!");
            Ok(SyncOutcome::Completed)
        }
        CheckSyncResult::FastForwardLocal => {
            pull_from_remote(&sync_config, &context, &remote_head, &main_sync_title)?;
            Ok(SyncOutcome::Completed)
        }
        CheckSyncResult::FastForwardRemote => {
            push_to_remote(&sync_config, &context, &remote_head, &main_sync_title)?;
            Ok(SyncOutcome::Completed)
        }
        CheckSyncResult::RemoteEmpty => {
            handle_remote_empty(&sync_config, &context, &remote_head, &main_sync_title)
        }
        CheckSyncResult::Conflict { local, remote } => handle_conflict(
            &sync_config,
            &context,
            &remote_head,
            &main_sync_title,
            &local,
            &remote,
        ),
    }
}

pub fn sync_thread_main(sync_key: &str, mut context: SyncThreadContext) {
    // Await for UI.
    block_until(&context.sync_rx, |cmd| {
        matches!(cmd, SyncThreadCommand::UIReady)
    });
    loop {
        match do_sync(&sync_key, &mut context) {
            Ok(SyncOutcome::Completed) => {
                // We're done - let UI thread exit with success
                context.send_ui_change_state(WebViewState::Success);
                std::thread::sleep(std::time::Duration::from_secs(1));
                let _ = context
                    .ui_proxy
                    .send_event(UIEvent::SyncSuccessCompletedEvent);
                break;
            }
            Ok(SyncOutcome::Cancelled) => {
                let _ = context.ui_proxy.send_event(UIEvent::SyncFailedEvent);
                break;
            }
            Err(e) => match handle_sync_error(&context, &e) {
                ErrorResolution::Retry => continue,
                ErrorResolution::Close => {
                    let _ = context.ui_proxy.send_event(UIEvent::SyncFailedEvent);
                    break;
                }
                ErrorResolution::ContinueOffline => {
                    let _ = context
                        .ui_proxy
                        .send_event(UIEvent::SyncSuccessCompletedEvent);
                    break;
                }
            },
        }
    }
}
