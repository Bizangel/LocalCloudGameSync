use crate::config::load_and_validate_config;
use crate::local_head;
use crate::remote_save_client::{RemoteSaveClient, get_default_remote_save_client};
use crate::tree_utils::tree_folder_hash;

#[derive(Debug, PartialEq)]
enum CheckSyncResult {
    FastForwardRemote,
    FastForwardLocal,
    UpToDate,
    Conflict,
}

impl CheckSyncResult {
    fn as_str(&self) -> &'static str {
        match self {
            CheckSyncResult::UpToDate => "UpToDate",
            CheckSyncResult::FastForwardLocal => "FastForwardLocal",
            CheckSyncResult::FastForwardRemote => "FastForwardRemote",
            CheckSyncResult::Conflict => "Conflict",
        }
    }
}

struct SyncStatusCheckInput<'a> {
    local_head: Option<&'a str>,
    current_head: &'a str,
    remote_head: Option<&'a str>,
}

pub fn check_sync_command(save_config_key: &String) -> Result<(), String> {
    let config = load_and_validate_config(save_config_key)?;
    let client = get_default_remote_save_client(&config);
    let local_head = local_head::read_local_head(&config.remote_sync_key)?;
    let current_head = tree_folder_hash(&config.local_save_folder, &config.ignore_globset)?;
    let remote_head = client.get_remote_head()?;

    let check_res = determine_sync_status(&SyncStatusCheckInput {
        local_head: local_head.as_deref(),
        current_head: &current_head,
        remote_head: remote_head.as_deref(),
    });

    todo!();

    // i need timestamps

    match check_res {
        CheckSyncResult::UpToDate => println!("UpToDate"),
        CheckSyncResult::FastForwardLocal => println!("UpToDate"),
        CheckSyncResult::FastForwardRemote => println!("UpToDate"),
        CheckSyncResult::Conflict => println!("Remote and local conflict found!"),
    }

    // println!("")

    Ok(())
}

fn determine_sync_status(input: &SyncStatusCheckInput) -> CheckSyncResult {
    let Some(remote_head) = input.remote_head else {
        return CheckSyncResult::FastForwardRemote; // no remote so just push
    };

    let Some(local_head) = input.local_head else {
        // remote exists - but local does not? Well then just pull
        return CheckSyncResult::FastForwardLocal;
    };

    if local_head == input.current_head {
        // no local changes
        if remote_head != local_head {
            // new remote changes - pull those.
            return CheckSyncResult::FastForwardLocal;
        }
        return CheckSyncResult::UpToDate; // all 3 are equal - no changes up to date
    }
    // We have local changes!
    if remote_head == local_head {
        // No changes on remote since last - time can safely push.
        return CheckSyncResult::FastForwardRemote;
    }
    // Local changes + Remote changes - user needs to pick
    return CheckSyncResult::Conflict;
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEAD: &str = "37df39a38c2f58ec73c309c67702de4d";
    const CHANGED_HEAD: &str = "ffa755f72c21bf534f54d3a2c75d4ed7";
    const CHANGED_HEAD2: &str = "19db3f74548df29b73598c030066b09d";

    #[test]
    pub fn initial_launch_empty_remote() {
        assert_eq!(
            determine_sync_status(&SyncStatusCheckInput {
                local_head: None,
                current_head: HEAD,
                remote_head: None
            }),
            CheckSyncResult::FastForwardRemote // if both local and remote repos are missing - should upload
        );
    }

    #[test]
    pub fn happy_path_single_device() {
        assert_eq!(
            determine_sync_status(&SyncStatusCheckInput {
                local_head: Some(HEAD),
                current_head: CHANGED_HEAD, // local and remote up to date - but local changes
                remote_head: Some(HEAD)
            }),
            CheckSyncResult::FastForwardRemote // should update remote
        );
    }

    #[test]
    pub fn happy_path_second_device_pull() {
        assert_eq!(
            determine_sync_status(&SyncStatusCheckInput {
                local_head: Some(HEAD),
                current_head: HEAD, // No local changes up to date - but remote has changes
                remote_head: Some(CHANGED_HEAD)
            }),
            CheckSyncResult::FastForwardLocal // should pull
        );
    }

    #[test]
    pub fn no_changes_equal_up_to_date() {
        assert_eq!(
            determine_sync_status(&SyncStatusCheckInput {
                local_head: Some(HEAD),
                current_head: HEAD, // No changes anywhere
                remote_head: Some(HEAD)
            }),
            CheckSyncResult::UpToDate
        );
    }

    #[test]
    pub fn conflict_when_both_changed() {
        assert_eq!(
            determine_sync_status(&SyncStatusCheckInput {
                local_head: Some(CHANGED_HEAD),
                current_head: HEAD,
                remote_head: Some(CHANGED_HEAD2)
            }),
            CheckSyncResult::Conflict
        );
    }
}
