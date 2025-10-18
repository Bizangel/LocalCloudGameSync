use crate::{
    common::Revision,
    ui::common::{ConflictDisplayInfo, UIEvent, WebViewCommand, WebViewState},
};
use tao::event_loop::EventLoopProxy;

pub(super) fn send_ui_display_update(
    ui_proxy: &EventLoopProxy<UIEvent>,
    titletext: impl Into<String>,
    subtext: impl Into<String>,
) {
    let cmd = UIEvent::WebViewCommand {
        command: WebViewCommand::WebViewUpdate {
            title_text: titletext.into(),
            sub_text: subtext.into(),
            conflict_info: None,
        },
    };
    let _ = ui_proxy.send_event(cmd);
}

pub(super) fn send_ui_display_update_conflict(
    ui_proxy: &EventLoopProxy<UIEvent>,
    title: &str,
    local: &Revision,
    remote: &Revision,
) {
    let conflict_info = ConflictDisplayInfo {
        local_modified_time: local.time_display_str(),
        remote_uploaded_time: remote.time_display_str(),
        local_author: local.author.clone(),
        remote_author: remote.author.clone(),
    };

    let cmd = UIEvent::WebViewCommand {
        command: WebViewCommand::WebViewUpdate {
            title_text: title.to_string(),
            sub_text: "".to_string(),
            conflict_info: Some(conflict_info),
        },
    };
    let _ = ui_proxy.send_event(cmd);
}

pub(super) fn send_ui_change_state(ui_proxy: &EventLoopProxy<UIEvent>, state: WebViewState) {
    let _ = ui_proxy.send_event(UIEvent::WebViewCommand {
        command: WebViewCommand::WebViewStateChange { state },
    });
}

pub(super) fn show_loading_step(ui_proxy: &EventLoopProxy<UIEvent>, title: &str, message: &str) {
    send_ui_change_state(ui_proxy, WebViewState::Loading);
    send_ui_display_update(ui_proxy, title, message);
}

pub(super) fn show_success_message(ui_proxy: &EventLoopProxy<UIEvent>, message: &str) {
    send_ui_display_update(ui_proxy, "Sync Success!", message);
}
