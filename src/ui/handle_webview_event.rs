use crate::ui::common::UserEvent;
use serde::Deserialize;
use tao::event_loop::EventLoopProxy;
use wry::http::Request;

#[derive(Deserialize)]
struct IPCWebViewEvent {
    command: String,
    #[allow(dead_code)]
    payload: Option<serde_json::Value>,
}

pub fn handle_webview_event(req: Request<String>, event_proxy: &EventLoopProxy<UserEvent>) {
    let Ok(ipc_req) = serde_json::from_str::<IPCWebViewEvent>(req.body()) else {
        return;
    };

    match ipc_req.command.as_str() {
        "sample-command" => {
            let _ = event_proxy.send_event(UserEvent::SampleCommand);
        }
        _ => println!("Received unknown command from rust!"),
    }
}
