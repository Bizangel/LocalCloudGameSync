use crate::ui::{
    common::{UIEvent, WebViewEvent, send_event_to_webview},
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
    sync_thread_handle: &RefCell<Option<JoinHandle<()>>>,
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
            UIEvent::WebViewReady => {
                // notify sync thread so it can start working.
                let _ = sync_tx.send(SyncThreadCommand::UIReady);
            }
            UIEvent::SyncSuccessCompletedEvent => {
                // successfully exit
                sync_thread_handle.borrow_mut().take().map(|t| t.join());
                *control_flow = ControlFlow::Exit; // exit code 0
            }
            UIEvent::ConflictResolve { choice } => {
                // send to sync thread
                let _ = sync_tx.send(SyncThreadCommand::ResolveConflict { choice });
            }
            UIEvent::WebViewUpdateRequest {
                title_text,
                sub_text,
            } => {
                // Forward it to webview
                send_event_to_webview(
                    &webview.borrow(),
                    &WebViewEvent::WebViewUpdate {
                        title_text,
                        sub_text,
                    },
                );
            }
            UIEvent::WebViewStateChangeRequest { state } => {
                // Forward it to webview
                send_event_to_webview(
                    &webview.borrow(),
                    &WebViewEvent::WebViewStateChange { state },
                );
            }
        },
        _ => {}
    }
}
