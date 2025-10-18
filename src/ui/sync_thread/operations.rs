use crate::{
    commands::{pull_command_with_update_callback, push_command_with_update_callback},
    common::Revision,
    config::RuntimeSyncConfig,
    ui::common::UIEvent,
};
use tao::event_loop::EventLoopProxy;

use super::ui_messages::{send_ui_display_update, show_loading_step, show_success_message};

pub(super) fn push_to_remote(
    sync_config: &RuntimeSyncConfig,
    ui_proxy: &EventLoopProxy<UIEvent>,
    remote_head: &Option<Revision>,
    main_sync_title: &str,
) -> Result<(), String> {
    show_loading_step(
        ui_proxy,
        main_sync_title,
        "Local changes found - saving to remote...",
    );

    let push_title = format!("Uploading {} save files", sync_config.game_display_name);
    push_command_with_update_callback(
        sync_config,
        remote_head.as_ref().map(|head| head.hash.as_str()),
        |txt| {
            send_ui_display_update(ui_proxy, &push_title, txt);
        },
    )?;

    show_success_message(ui_proxy, "Uploaded to remote!");

    Ok(())
}

pub(super) fn pull_from_remote(
    sync_config: &RuntimeSyncConfig,
    ui_proxy: &EventLoopProxy<UIEvent>,
    remote_head: &Option<Revision>,
    main_sync_title: &str,
) -> Result<(), String> {
    show_loading_step(
        ui_proxy,
        main_sync_title,
        "Newer version on remote found! Downloading from remote...",
    );

    let pull_title = format!("Downloading {} save files", sync_config.game_display_name);
    pull_command_with_update_callback(
        sync_config,
        remote_head.as_ref().map(|head| head.hash.as_str()),
        |txt| {
            send_ui_display_update(ui_proxy, &pull_title, txt);
        },
    )?;

    show_success_message(ui_proxy, "Downloaded from remote!");

    Ok(())
}
