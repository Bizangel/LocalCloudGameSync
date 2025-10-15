pub const UI_TITLE_NAME: &str = "Local Cloud Game Sync";
pub const UI_INITIAL_SIZE_WIDTH_PX: f64 = 800.0;
pub const UI_INITIAL_SIZE_HEIGHT_PX: f64 = 600.0;
pub const VITE_DEV_LOCALHOST_URL: &str = "http://localhost:5173";
use serde::Serialize;
use wry::WebView;

#[derive(Debug, Clone)]
pub enum ResolveConflictChoice {
    Push,
    Pull,
}

// Events generated to be handled for the main loop
pub enum UIEvent {
    SampleCommand,

    SyncDoneEvent,

    ConflictResolve { choice: ResolveConflictChoice },
}

// Events generated from rust code to be posted to the webview.
#[derive(Serialize)]
pub enum WebViewEvent {
    WebViewUpdate { displaystring: String },
}

pub fn send_event_to_webview(webview: &WebView, ev: &WebViewEvent) {
    let Ok(evpayload) = serde_json::to_string(&ev) else {
        return;
    };

    let script = format!("window.postMessage({}, '*');", evpayload);
    let _ = webview.evaluate_script(&script);
}
