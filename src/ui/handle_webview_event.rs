use crate::ui::common::{
    ResolveConflictChoice, ResolveErrorChoice, UIEvent, WebViewRequest, WebViewRequestType,
};
use tao::event_loop::EventLoopProxy;
use wry::http::Request;

pub fn handle_webview_event(req: Request<String>, event_proxy: &EventLoopProxy<UIEvent>) {
    let Ok(ipc_req) = serde_json::from_str::<WebViewRequest>(req.body()) else {
        return;
    };

    let Ok(req_type) = ipc_req.request.parse::<WebViewRequestType>() else {
        return;
    };

    match req_type {
        WebViewRequestType::WebViewReady => {
            // let event loop know webview is ready
            let _ = event_proxy.send_event(UIEvent::WebViewReady);
        }
        WebViewRequestType::ResolveConflict => {
            if let Ok(choice) = ipc_req.body.parse::<ResolveConflictChoice>() {
                let _ = event_proxy.send_event(UIEvent::ResolveConflict { choice });
            };
        }
        WebViewRequestType::ResolveError => {
            if let Ok(choice) = ipc_req.body.parse::<ResolveErrorChoice>() {
                let _ = event_proxy.send_event(UIEvent::ResolveError { choice });
            };
        }
    }
}
