use crate::ui::{
    common::{RustWebViewEvent, UserEvent, send_event_to_webview},
    handle_window_event::handle_window_event,
};
use std::{cell::RefCell, rc::Rc};
use tao::{event::Event, event_loop::ControlFlow, window::Window};

use wry::WebView;

pub fn handle_main_loop_event(
    event: Event<UserEvent>,
    control_flow: &mut ControlFlow,
    webview: &Rc<RefCell<WebView>>,
    window: &Window,
) {
    match event {
        Event::WindowEvent { event, .. } => {
            handle_window_event(&event, control_flow, window, webview)
        }
        Event::UserEvent(event) => match event {
            UserEvent::SampleCommand => {
                println!("Sample Command");

                // return to js
                let replyevent = RustWebViewEvent::SampleWebviewUpdate {
                    displaystring: "samplestring".to_string(),
                };
                send_event_to_webview(&webview.borrow(), &replyevent);
            }
        },
        _ => {}
    }
}
