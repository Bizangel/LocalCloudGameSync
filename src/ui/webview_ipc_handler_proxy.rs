use std::str::FromStr;

use crate::ui::common::{UIEvent, UserChoice, WebViewEvent};
use serde::Deserialize;
use tao::event_loop::EventLoopProxy;
use wry::http::Request;

#[derive(Deserialize, Debug)]
pub struct WebViewIPCEvent {
    pub event_type: String,
    pub body: String,
}

impl WebViewIPCEvent {
    fn parse_ipc_req(self) -> Result<WebViewEvent, ()> {
        match self.event_type.to_lowercase().as_str() {
            "webview-ready" => Ok(WebViewEvent::WebViewReady),
            "user-choice" => Ok(WebViewEvent::UserChoice {
                choice: self.body.parse::<UserChoice>()?,
            }),
            _ => Err(()),
        }
    }
}

impl FromStr for UserChoice {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "push" => Ok(UserChoice::Push),
            "pull" => Ok(UserChoice::Pull),
            "retry" => Ok(UserChoice::Retry),
            "close" => Ok(UserChoice::Close),
            "continue-offline" => Ok(UserChoice::ContinueOffline),
            _ => Err(()),
        }
    }
}

pub fn proxy_webview_event_to_event_loop(
    req: Request<String>,
    event_proxy: &EventLoopProxy<UIEvent>,
) {
    let Ok(ipc_req) = serde_json::from_str::<WebViewIPCEvent>(req.body()) else {
        return;
    };

    let Ok(event) = ipc_req.parse_ipc_req() else {
        return;
    };

    // forward webview events to main UI
    let _ = event_proxy.send_event(UIEvent::WebViewEvent { event: event });
}
