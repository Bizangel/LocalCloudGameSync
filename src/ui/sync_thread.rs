use crate::{
    commands::{CheckSyncResult, check_sync_command},
    config::RuntimeSyncConfig,
    ui::common::{ResolveConflictChoice, UIEvent},
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
}

fn send_ui_display_update(ui_proxy: &EventLoopProxy<UIEvent>, display_text: impl Into<String>) {
    let req = UIEvent::WebViewUpdateRequest {
        display_text: display_text.into(),
    };
    let _ = ui_proxy.send_event(req);
}

pub fn do_sync(
    sync_config: &RuntimeSyncConfig,
    ui_proxy: &EventLoopProxy<UIEvent>,
    sync_rx: &Receiver<SyncThreadCommand>,
) -> Result<(), String> {
    // Await for UI.
    block_until(&sync_rx, |cmd| matches!(cmd, SyncThreadCommand::UIReady));

    send_ui_display_update(&ui_proxy, "Checking remote...");

    let check_sync_result = check_sync_command(&sync_config, false)?;

    match check_sync_result {
        CheckSyncResult::UpToDate => {
            send_ui_display_update(&ui_proxy, "Local is up to date!");
            return Ok(());
        }
        CheckSyncResult::FastForwardLocal => {
            send_ui_display_update(
                &ui_proxy,
                "Newer version on remote found! Pulling from remote...",
            );
            // TODO: take action
            return Ok(());
        }
        CheckSyncResult::FastForwardRemote => {
            send_ui_display_update(&ui_proxy, "Local changes found - saving to remote...");
            // TODO: take action
            return Ok(());
        }
        CheckSyncResult::RemoteEmpty => {
            send_ui_display_update(
                &ui_proxy,
                format!(
                    "Remote for key {} empty! Do you wish to push to initialize remote!",
                    &sync_config.remote_sync_key
                ),
            );

            // TODO: take action

            return Ok(());
        }
        CheckSyncResult::Conflict => {
            send_ui_display_update(&ui_proxy, format!("Conflict found for key! Found: "));

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
    match do_sync(&sync_config, &ui_proxy, &sync_rx) {
        Ok(_) => {
            // We're done - let UI thread exit with success
            // wait 2 seconds so user can read
            std::thread::sleep(std::time::Duration::from_secs(2));
            let _ = ui_proxy.send_event(UIEvent::SyncSuccessCompletedEvent);
        }
        Err(e) => {
            // Error send error
            send_ui_display_update(&ui_proxy, format!("Error: {}", e));
        }
    }
}
