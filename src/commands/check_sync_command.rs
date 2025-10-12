use crate::config::RuntimeSyncConfig;
use crate::local_head;
use crate::remote_save_client::{RemoteSaveClient, get_default_remote_save_client};
use crate::tree_utils::tree_folder_hash;

#[derive(Debug, PartialEq)]
pub enum CheckSyncResult {
    FastForwardRemote,
    FastForwardLocal,
    UpToDate,
    Conflict,
    RemoteEmpty,
}

impl CheckSyncResult {
    fn as_str(&self) -> &'static str {
        match self {
            CheckSyncResult::UpToDate => "UpToDate",
            CheckSyncResult::RemoteEmpty => "RemoteEmpty",
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

pub fn check_sync_command(
    sync_config: &RuntimeSyncConfig,
    short_flag: bool,
) -> Result<CheckSyncResult, String> {
    let client = get_default_remote_save_client(&sync_config);
    let local_head = local_head::read_local_head(&sync_config)?;
    let current_head =
        tree_folder_hash(&sync_config.local_save_folder, &sync_config.ignore_globset)?;
    let remote_head = client.get_remote_head()?;

    let check_res = determine_sync_status(&SyncStatusCheckInput {
        local_head: local_head.as_ref().map(|h| h.hash.as_str()),
        current_head: &current_head,
        remote_head: remote_head.as_ref().map(|h| h.hash.as_str()),
    });

    if short_flag {
        println!("{}", check_res.as_str());
        return Ok(check_res);
    }

    let local_head_display = local_head
        .as_ref()
        .map(|x| x.to_string())
        .unwrap_or_default();

    let remote_head_display = remote_head
        .as_ref()
        .map(|x| x.to_string())
        .unwrap_or_default();

    match check_res {
        CheckSyncResult::UpToDate => {
            println!("Already up to date! Current revision {current_head}")
        }
        CheckSyncResult::RemoteEmpty => {
            println!(
                "Remote is empty - no HEAD found for remote for key {}. Will upload local head.\nLocal: {} ",
                sync_config.remote_sync_key, current_head
            )
        }
        CheckSyncResult::FastForwardLocal => {
            println!(
                "Local is out of date - new version on remote - will pull from remote.\nLocal: {} Remote: {}",
                local_head_display, remote_head_display
            )
        }
        CheckSyncResult::FastForwardRemote => println!(
            "Remote is out of date - new version locally - will push to remote.\nLocal: {} Remote: {}",
            current_head, remote_head_display
        ),
        CheckSyncResult::Conflict => {
            println!(
                "Conflict found - both remote and local have updates.\nLocal: {} Remote: {}",
                local_head_display, remote_head_display
            )
        }
    }

    Ok(check_res)
}

fn determine_sync_status(input: &SyncStatusCheckInput) -> CheckSyncResult {
    let Some(remote_head) = input.remote_head else {
        return CheckSyncResult::RemoteEmpty; // no remote so just push
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
            CheckSyncResult::RemoteEmpty // if both local and remote repos are missing - should upload
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
