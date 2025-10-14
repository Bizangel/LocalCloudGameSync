use crate::ui::{
    common::{RustWebViewEvent, UIEvent, send_event_to_webview},
    handle_window_event::handle_window_event,
    sync_thread::SyncThreadCommand,
};
use std::{cell::RefCell, rc::Rc, sync::mpsc::Sender, thread::JoinHandle};
use tao::{event::Event, event_loop::ControlFlow, window::Window};

use wry::WebView;

pub fn handle_main_loop_event(
    event: Event<UIEvent>,
    control_flow: &mut ControlFlow,
    webview: &Rc<RefCell<WebView>>,
    window: &Window,
    sync_tx: &Sender<SyncThreadCommand>,
    sync_thread_handle: &Rc<RefCell<Option<JoinHandle<()>>>>,
) {
    match event {
        Event::WindowEvent { event, .. } => handle_window_event(
            &event,
            control_flow,
            window,
            webview,
            sync_tx,
            sync_thread_handle,
        ),
        Event::UserEvent(event) => match event {
            UIEvent::SampleCommand => {
                println!("Sample Command");

                // return to js
                let replyevent = RustWebViewEvent::SampleWebviewUpdate {
                    displaystring: "samplestring".to_string(),
                };
                send_event_to_webview(&webview.borrow(), &replyevent);
            }
            UIEvent::SyncDoneEvent => {
                // successfully exit
                if let Some(handle) = sync_thread_handle.borrow_mut().take() {
                    let _ = handle.join(); // TODO: Attempt to handle errors gracefully.
                };
                *control_flow = ControlFlow::Exit; // exit code 0
            }
            UIEvent::ConflictResolve { choice } => {
                // send to sync thread
                let _ = sync_tx.send(SyncThreadCommand::ResolveConflict { choice });
            }
        },
        _ => {}
    }
}
