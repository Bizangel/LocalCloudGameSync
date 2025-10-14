use std::sync::mpsc::Receiver;

use tao::event_loop::EventLoopProxy;

use crate::ui::common::{ResolveConflictChoice, UIEvent};

#[derive(Debug, Clone)]
pub enum SyncThreadCommand {
    ResolveConflict { choice: ResolveConflictChoice },
}

pub fn sync_thread(ui_proxy: EventLoopProxy<UIEvent>, cmd_receiver: Receiver<SyncThreadCommand>) {
    println!("Starting syncing!");

    println!("Awaiting until resolve!");

    // TODO: Handle this more gracefully.
    let choice = cmd_receiver.recv().unwrap();

    // std::thread::sleep(std::time::Duration::from_secs(10));

    println!("Done syncing with choice: {:?}", choice);
    let _ = ui_proxy.send_event(UIEvent::SyncDoneEvent);
}
