pub const UI_TITLE_NAME: &str = "Local Cloud Game Sync";
pub const UI_INITIAL_SIZE_WIDTH_PX: f64 = 1000.0;
pub const UI_INITIAL_SIZE_HEIGHT_PX: f64 = 720.0;
pub const VITE_DEV_LOCALHOST_URL: &str = "http://localhost:5173";
use serde::Serialize;
use std::{path::PathBuf, sync::mpsc::Receiver};
use tao::event_loop::EventLoopProxy;
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

#[derive(Serialize, Debug, Clone)]
pub struct ConflictDisplayInfo {
    pub local_modified_time: String,
    pub remote_uploaded_time: String,
    pub local_author: String,
    pub remote_author: String,
}

// Commands generated from rust code to be processed to the webview.
#[derive(Serialize, Debug, Clone)]
pub enum WebViewCommand {
    WebViewUpdate {
        title_text: String,
        sub_text: String,
        conflict_info: Option<ConflictDisplayInfo>,
        is_after_game: bool,
    },

    WebViewStateChange {
        state: WebViewState,
    },

    // Notifies the webview when the user is attempting to close the window.
    WebViewNotifyClose {
        empty_payload: u32, // needs something else parsing logic breaks
    },
}

pub fn send_event_to_webview(webview: &WebView, ev: &WebViewCommand) {
    let Ok(evpayload) = serde_json::to_string(&ev) else {
        return;
    };

    let script = format!("window.postMessage({}, '*');", evpayload);
    let _ = webview.evaluate_script(&script);
}

/// Commands sent from main UI loop to the Sync Thread.
#[derive(Debug, Clone)]
pub enum SyncThreadCommand {
    UIReady,
    UserChoice { choice: UserChoice },
}

pub struct SyncThreadContext {
    pub ui_proxy: EventLoopProxy<UIEvent>,
    pub sync_rx: Receiver<SyncThreadCommand>,
    pub after_game: bool,
    pub config_file_override: Option<PathBuf>,
    // TODO: This shouldn't be here but I don't wanna refactor further. This is just to display game name best-effort on error.
    // Game name cannot be displayed if the error itself was loading the config.
    pub game_display_name: Option<String>,
}
