use std::{cell::RefCell, rc::Rc, sync::mpsc::Sender, thread::JoinHandle};
use wry::{
    Rect, WebView,
    dpi::{LogicalPosition, LogicalSize},
};

use tao::{event::WindowEvent, event_loop::ControlFlow, keyboard::Key, window::Window};

use crate::ui::{
    common::{ResolveErrorChoice, WebViewState},
    sync_thread::SyncThreadCommand,
};

pub fn handle_window_event(
    event: &WindowEvent,
    control_flow: &mut ControlFlow,
    window: &Window,
    webview: &Rc<RefCell<WebView>>,
    sync_tx: &Sender<SyncThreadCommand>,
    sync_thread_handle: &RefCell<Option<JoinHandle<()>>>,
    current_state: &RefCell<WebViewState>,
) {
    match event {
        WindowEvent::Resized(new_size) => {
            let logical_size = new_size.to_logical::<f64>(window.scale_factor());
            webview
                .borrow()
                .set_bounds(Rect {
                    position: LogicalPosition::new(0, 0).into(),
                    size: LogicalSize::new(logical_size.width, logical_size.height).into(),
                })
                .unwrap();
        }
        WindowEvent::KeyboardInput { event, .. } => {
            let key = &event.logical_key;
            match key {
                Key::Character("i") => {
                    #[cfg(debug_assertions)]
                    {
                        webview.borrow().open_devtools();
                    }
                }
                _ => {}
            }
        }
        WindowEvent::CloseRequested => {
            // If loading - prevent closing
            if let WebViewState::Loading = *current_state.borrow() {
                return;
            };

            // TODO: Handle conflict state properly - as probably has diff logic
            let _ = sync_tx.send(SyncThreadCommand::ResolveError {
                choice: ResolveErrorChoice::Close,
            });

            sync_thread_handle.borrow_mut().take().map(|t| t.join());
            *control_flow = ControlFlow::ExitWithCode(1);
        }
        _ => {}
    }
}
