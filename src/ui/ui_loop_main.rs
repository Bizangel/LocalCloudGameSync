use std::{
    cell::RefCell,
    sync::mpsc::{self, Receiver, Sender},
};

use tao::event_loop::EventLoopBuilder;
use wry::http::Request;

use crate::{
    config::RuntimeSyncConfig,
    ui::{
        common::{UIEvent, WebViewState},
        handle_main_loop_event::handle_main_loop_event,
        sync_thread::{SyncThreadCommand, sync_thread_main},
        webview_ipc_handler_proxy::proxy_webview_event_to_event_loop,
        window_builder::build_window_with_webview,
    },
};

pub fn ui_loop_main(sync_config: RuntimeSyncConfig) -> Result<(), String> {
    let event_loop = EventLoopBuilder::<UIEvent>::with_user_event().build();
    let event_proxy = event_loop.create_proxy();
    let sync_thread_event_proxy = event_proxy.clone();

    let webview_ipc_handler = move |req: Request<String>| {
        proxy_webview_event_to_event_loop(req, &event_proxy);
    };

    let (window, webview) = build_window_with_webview(&event_loop, webview_ipc_handler);
    let (sync_tx, sync_rx): (Sender<SyncThreadCommand>, Receiver<SyncThreadCommand>) =
        mpsc::channel();

    // Spawn actual sync thread
    let sync_thread_handle = std::thread::spawn(move || {
        sync_thread_main(sync_config, sync_thread_event_proxy, sync_rx);
    });
    let sync_thread_handle = RefCell::new(Some(sync_thread_handle));
    let webview_state = RefCell::new(WebViewState::Loading);

    event_loop.run(move |event, _, control_flow| {
        handle_main_loop_event(
            event,
            control_flow,
            &webview,
            &window,
            &sync_tx,
            &sync_thread_handle,
            &webview_state,
        );
    });
}
