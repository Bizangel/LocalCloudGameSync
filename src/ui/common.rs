pub const UI_TITLE_NAME: &str = "Local Cloud Game Sync";
pub const UI_INITIAL_SIZE_WIDTH_PX: f64 = 800.0;
pub const UI_INITIAL_SIZE_HEIGHT_PX: f64 = 600.0;
pub const VITE_DEV_LOCALHOST_URL: &str = "http://localhost:5173";
use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;
use wry::WebView;

#[derive(Debug, Clone)]
pub enum ResolveConflictChoice {
    Push,
    Pull,
}

impl FromStr for ResolveConflictChoice {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "push" => Ok(ResolveConflictChoice::Push),
            "pull" => Ok(ResolveConflictChoice::Pull),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum WebViewRequestType {
    WebViewReady,
    ResolveConflict,
}

#[derive(Deserialize, Debug)]
/// Requests coming from webview into main UI event loop
pub struct WebViewRequest {
    pub request: String,
    pub body: String,
}

impl FromStr for WebViewRequestType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "webview-ready" => Ok(WebViewRequestType::WebViewReady),
            "resolve-conflict" => Ok(WebViewRequestType::ResolveConflict),
            _ => Err(()),
        }
    }
}

// Events generated to be handled for the main loop
pub enum UIEvent {
    WebViewReady,
    SyncSuccessCompletedEvent,
    WebViewUpdateRequest { display_text: String },

    ConflictResolve { choice: ResolveConflictChoice },
}

// Events generated from rust code to be posted to the webview.
#[derive(Serialize)]
pub enum WebViewEvent {
    WebViewUpdate { display_text: String },
}

pub fn send_event_to_webview(webview: &WebView, ev: &WebViewEvent) {
    let Ok(evpayload) = serde_json::to_string(&ev) else {
        return;
    };

    let script = format!("window.postMessage({}, '*');", evpayload);
    let _ = webview.evaluate_script(&script);
}
