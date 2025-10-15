use crate::ui::common::{ResolveConflictChoice, UIEvent};
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

pub fn sync_thread(ui_proxy: EventLoopProxy<UIEvent>, sync_rx: Receiver<SyncThreadCommand>) {
    println!("Awaiting until UI is ready!");
    block_until(&sync_rx, |cmd| matches!(cmd, SyncThreadCommand::UIReady));
    println!("UI is ready!");

    let x = block_until(&sync_rx, |cmd| {
        matches!(cmd, SyncThreadCommand::ResolveConflict { choice: _ })
    });

    // std::thread::sleep(std::time::Duration::from_secs(10));

    println!("Done syncing with choice: {:?}", x);
    let _ = ui_proxy.send_event(UIEvent::SyncDoneEvent);
}
