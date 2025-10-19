use crate::{
    commands::{pull_command_with_update_callback, push_command_with_update_callback},
    common::Revision,
    config::RuntimeSyncConfig,
    ui::common::SyncThreadContext,
};

pub(super) fn push_to_remote(
    sync_config: &RuntimeSyncConfig,
    context: &SyncThreadContext,
    remote_head: &Option<Revision>,
    main_sync_title: &str,
) -> Result<(), String> {
    context.show_loading_step(main_sync_title, "Local changes found - saving to remote...");

    let push_title = format!("Uploading {} save files", sync_config.game_display_name);
    push_command_with_update_callback(
        sync_config,
        remote_head.as_ref().map(|head| head.hash.as_str()),
        |txt| {
            context.send_ui_display_update(&push_title, txt);
        },
    )?;

    context.show_success_message(&sync_config.game_display_name, "Uploaded to remote!");

    Ok(())
}

pub(super) fn pull_from_remote(
    sync_config: &RuntimeSyncConfig,
    context: &SyncThreadContext,
    remote_head: &Option<Revision>,
    main_sync_title: &str,
) -> Result<(), String> {
    context.show_loading_step(
        main_sync_title,
        "Newer version on remote found! Downloading from remote...",
    );

    let pull_title = format!("Downloading {} save files", sync_config.game_display_name);
    pull_command_with_update_callback(
        sync_config,
        remote_head.as_ref().map(|head| head.hash.as_str()),
        |txt| {
            context.send_ui_display_update(&pull_title, txt);
        },
    )?;

    context.show_success_message(&sync_config.game_display_name, "Downloaded from remote!");

    Ok(())
}
