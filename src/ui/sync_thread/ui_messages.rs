use crate::{
    common::Revision,
    ui::common::{ConflictDisplayInfo, SyncThreadContext, UIEvent, WebViewCommand, WebViewState},
};

impl SyncThreadContext {
    pub(super) fn send_ui_display_update(
        &self,
        titletext: impl Into<String>,
        subtext: impl Into<String>,
    ) {
        let cmd = UIEvent::WebViewCommand {
            command: WebViewCommand::WebViewUpdate {
                title_text: titletext.into(),
                sub_text: subtext.into(),
                conflict_info: None,
                is_after_game: self.after_game,
            },
        };
        let _ = self.ui_proxy.send_event(cmd);
    }

    pub(super) fn send_ui_display_update_conflict(
        &self,
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
                is_after_game: self.after_game,
            },
        };
        let _ = self.ui_proxy.send_event(cmd);
    }

    pub(super) fn send_ui_change_state(&self, state: WebViewState) {
        let _ = self.ui_proxy.send_event(UIEvent::WebViewCommand {
            command: WebViewCommand::WebViewStateChange { state },
        });
    }

    pub(super) fn show_loading_step(&self, title: &str, message: &str) {
        self.send_ui_change_state(WebViewState::Loading);
        self.send_ui_display_update(title, message);
    }

    pub(super) fn show_success_message(&self, display_name: &str, message: &str) {
        self.send_ui_display_update(format!("{} Sync Success!", display_name), message);
    }
}
