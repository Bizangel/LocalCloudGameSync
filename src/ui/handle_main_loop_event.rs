use crate::ui::{
    common::{
        SyncThreadCommand, UIEvent, WebViewCommand, WebViewEvent, WebViewState,
        send_event_to_webview,
    },
    handle_window_event::handle_window_event,
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
    current_state: &RefCell<WebViewState>,
) {
    match event {
        Event::WindowEvent { event, .. } => handle_window_event(
            &event,
            control_flow,
            window,
            webview,
            sync_tx,
            sync_thread_handle,
            &current_state,
        ),
        Event::UserEvent(event) => match event {
            UIEvent::WebViewEvent { event } => {
                match event {
                    WebViewEvent::WebViewReady => {
                        // notify sync thread so it can start working.
                        let _ = sync_tx.send(SyncThreadCommand::UIReady);
                    }
                    WebViewEvent::UserChoice { choice } => {
                        let _ = sync_tx.send(SyncThreadCommand::UserChoice { choice });
                    }
                }
            }
            UIEvent::SyncSuccessCompletedEvent => {
                sync_thread_handle.borrow_mut().take().map(|t| t.join());
                *control_flow = ControlFlow::Exit; // exit code 0
            }
            UIEvent::SyncFailedEvent => {
                sync_thread_handle.borrow_mut().take().map(|t| t.join());
                *control_flow = ControlFlow::ExitWithCode(1); // exit non-zero
            }
            UIEvent::WebViewCommand { command } => {
                // if it's state change - also update internal state
                match command {
                    WebViewCommand::WebViewStateChange { ref state } => {
                        *current_state.borrow_mut() = state.clone();
                    }
                    _ => {}
                }

                // Forward it to webview
                send_event_to_webview(&webview.borrow(), &command);
            }
        },
        _ => {}
    }
}
