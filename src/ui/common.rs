pub const UI_TITLE_NAME: &str = "Local Cloud Game Sync";
pub const UI_INITIAL_SIZE_WIDTH_PX: f64 = 800.0;
pub const UI_INITIAL_SIZE_HEIGHT_PX: f64 = 600.0;
pub const VITE_DEV_LOCALHOST_URL: &str = "http://localhost:5173";
use serde::Serialize;
use wry::WebView;

#[derive(Debug, Clone)]
pub enum UserChoice {
    Push,
    Pull,
    Retry,
    Close,
    ContinueOffline,
}

#[derive(Debug, Clone, Serialize)]
pub enum WebViewState {
    Loading,     // Default loading state
    Conflict,    // A conflict has ocurred - user needs to make choice
    Error,       // An error has ocurred - user needs to make choice - or retry.
    Success,     // Small green check after success
    RemoteEmpty, // Remote repository is empty - confirm push
}

/// IPC Events generated from webview to rust.
#[derive(Debug, Clone)]
pub enum WebViewEvent {
    WebViewReady,
    UserChoice { choice: UserChoice },
}

// Events generated to be handled for the main loop
#[derive(Debug, Clone)]
pub enum UIEvent {
    WebViewEvent { event: WebViewEvent },
    SyncSuccessCompletedEvent,
    SyncFailedEvent,

    WebViewCommand { command: WebViewCommand },
}

// Commands generated from rust code to be processed to the webview.
#[derive(Serialize, Debug, Clone)]
pub enum WebViewCommand {
    WebViewUpdate {
        title_text: String,
        sub_text: String,
    },

    WebViewStateChange {
        state: WebViewState,
    },
}

pub fn send_event_to_webview(webview: &WebView, ev: &WebViewCommand) {
    let Ok(evpayload) = serde_json::to_string(&ev) else {
        return;
    };

    let script = format!("window.postMessage({}, '*');", evpayload);
    let _ = webview.evaluate_script(&script);
}
