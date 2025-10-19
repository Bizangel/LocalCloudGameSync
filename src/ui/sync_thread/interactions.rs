use crate::{
    common::Revision,
    config::RuntimeSyncConfig,
    ui::common::{UIEvent, UserChoice, WebViewState},
};
use std::sync::mpsc::Receiver;
use tao::event_loop::EventLoopProxy;

use super::{
    SyncThreadCommand,
    operations::{pull_from_remote, push_to_remote},
    ui_messages::{send_ui_change_state, send_ui_display_update, send_ui_display_update_conflict},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SyncOutcome {
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ErrorResolution {
    Retry,
    Close,
    ContinueOffline,
}

fn wait_for_user_choice<F>(sync_rx: &Receiver<SyncThreadCommand>, mut predicate: F) -> UserChoice
where
    F: FnMut(UserChoice) -> Option<UserChoice>,
{
    loop {
        match sync_rx.recv() {
            Ok(SyncThreadCommand::UserChoice { choice }) => {
                if let Some(choice) = predicate(choice) {
                    return choice;
                }
            }
            Ok(_) => continue,
            Err(e) => panic!("{e}"),
        }
    }
}

pub(super) fn handle_remote_empty(
    sync_config: &RuntimeSyncConfig,
    ui_proxy: &EventLoopProxy<UIEvent>,
    sync_rx: &Receiver<SyncThreadCommand>,
    remote_head: &Option<Revision>,
    main_sync_title: &str,
) -> Result<SyncOutcome, String> {
    send_ui_change_state(ui_proxy, WebViewState::RemoteEmpty);
    send_ui_display_update(
        ui_proxy,
        format!("{} Upload Confirmation", sync_config.game_display_name),
        format!(
            "Remote for key {} is empty! Do you wish to push to initialize remote?",
            sync_config.remote_sync_key
        ),
    );

    let choice = wait_for_user_choice(sync_rx, |choice| match choice {
        UserChoice::Push | UserChoice::Close => Some(choice),
        _ => None,
    });

    match choice {
        UserChoice::Push => {
            push_to_remote(sync_config, ui_proxy, remote_head, main_sync_title)?;
            Ok(SyncOutcome::Completed)
        }
        UserChoice::Close => Ok(SyncOutcome::Cancelled),
        _ => unreachable!(),
    }
}

pub(super) fn handle_conflict(
    sync_config: &RuntimeSyncConfig,
    ui_proxy: &EventLoopProxy<UIEvent>,
    sync_rx: &Receiver<SyncThreadCommand>,
    remote_head: &Option<Revision>,
    main_sync_title: &str,
    local: &Revision,
    remote: &Revision,
) -> Result<SyncOutcome, String> {
    send_ui_change_state(ui_proxy, WebViewState::Conflict);
    send_ui_display_update_conflict(
        ui_proxy,
        &format!("{} Conflict Found", sync_config.game_display_name),
        local,
        remote,
    );

    let choice = wait_for_user_choice(sync_rx, |choice| match choice {
        UserChoice::Pull | UserChoice::Push | UserChoice::Close => Some(choice),
        _ => None,
    });

    match choice {
        UserChoice::Pull => {
            pull_from_remote(sync_config, ui_proxy, remote_head, main_sync_title)?;
            Ok(SyncOutcome::Completed)
        }
        UserChoice::Push => {
            push_to_remote(sync_config, ui_proxy, remote_head, main_sync_title)?;
            Ok(SyncOutcome::Completed)
        }
        UserChoice::Close => Ok(SyncOutcome::Cancelled),
        _ => unreachable!(),
    }
}

pub(super) fn handle_sync_error(
    ui_proxy: &EventLoopProxy<UIEvent>,
    sync_rx: &Receiver<SyncThreadCommand>,
    game_display_name: &str,
    error_message: &str,
) -> ErrorResolution {
    send_ui_change_state(ui_proxy, WebViewState::Error);
    send_ui_display_update(
        ui_proxy,
        format!("{} Sync Error", game_display_name),
        error_message,
    );

    let choice = wait_for_user_choice(sync_rx, |choice| match choice {
        UserChoice::Close | UserChoice::ContinueOffline | UserChoice::Retry => Some(choice),
        _ => None,
    });

    match choice {
        UserChoice::Close => ErrorResolution::Close,
        UserChoice::ContinueOffline => ErrorResolution::ContinueOffline,
        UserChoice::Retry => ErrorResolution::Retry,
        _ => unreachable!(),
    }
}
