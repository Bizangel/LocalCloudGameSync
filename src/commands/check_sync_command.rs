use crate::common::Revision;
use crate::config::RuntimeSyncConfig;
use crate::local_head;
use crate::remote_save_client::{RemoteSaveClient, get_default_remote_save_client};

#[derive(Debug)]
pub enum CheckSyncResult {
    FastForwardRemote,
    FastForwardLocal,
    UpToDate,
    Conflict { local: Revision, remote: Revision },
    RemoteEmpty,
}

struct SyncStatusCheckInput<'a> {
    local_head: &'a Option<Revision>,
    current_head: &'a Revision,
    remote_head: &'a Option<Revision>,
}

pub fn check_sync_command(
    sync_config: &RuntimeSyncConfig,
) -> Result<(CheckSyncResult, Option<Revision>), String> {
    let client = get_default_remote_save_client(&sync_config);
    let local_head = local_head::read_local_head(&sync_config)?;
    let current_head = local_head::generate_current_head(
        &sync_config.local_save_folder,
        &sync_config.ignore_globset,
    )?;
    let remote_head = client.get_remote_head()?;

    let check_res = determine_sync_status(&SyncStatusCheckInput {
        local_head: &local_head,
        current_head: &current_head,
        remote_head: &remote_head,
    });

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
        CheckSyncResult::Conflict {
            ref local,
            ref remote,
        } => {
            println!(
                "Conflict found - both remote and local have updates.\nLocal: {} Remote: {}",
                &local, &remote
            )
        }
    }

    Ok((check_res, remote_head))
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
    return CheckSyncResult::Conflict {
        local: input.current_head.clone(),
        remote: remote_head.clone(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn head() -> Revision {
        return Revision {
            hash: "37df39a38c2f58ec73c309c67702de4d".to_string(),
            timestamp: 1760783380,
        };
    }

    fn changed_head() -> Revision {
        return Revision {
            hash: "ffa755f72c21bf534f54d3a2c75d4ed7".to_string(),
            timestamp: 1760789280,
        };
    }

    fn changed_head2() -> Revision {
        return Revision {
            hash: "19db3f74548df29b73598c030066b09d".to_string(),
            timestamp: 1760793480,
        };
    }

    #[test]
    pub fn initial_launch_empty_remote() {
        assert!(matches!(
            determine_sync_status(&SyncStatusCheckInput {
                local_head: &None,
                current_head: &head(),
                remote_head: &None
            }),
            CheckSyncResult::RemoteEmpty // if both local and remote repos are missing - should upload
        ));
    }

    #[test]
    pub fn happy_path_single_device() {
        assert!(matches!(
            determine_sync_status(&SyncStatusCheckInput {
                local_head: &Some(head()),
                current_head: &changed_head(), // local and remote up to date - but local changes
                remote_head: &Some(head())
            }),
            CheckSyncResult::FastForwardRemote // should update remote
        ));
    }

    #[test]
    pub fn happy_path_second_device_pull() {
        assert!(matches!(
            determine_sync_status(&SyncStatusCheckInput {
                local_head: &Some(head()),
                current_head: &head(), // No local changes up to date - but remote has changes
                remote_head: &Some(changed_head())
            }),
            CheckSyncResult::FastForwardLocal // should pull
        ));
    }

    #[test]
    pub fn no_changes_equal_up_to_date() {
        assert!(matches!(
            determine_sync_status(&SyncStatusCheckInput {
                local_head: &Some(head()),
                current_head: &head(), // No changes anywhere
                remote_head: &Some(head())
            }),
            CheckSyncResult::UpToDate
        ));
    }

    #[test]
    pub fn conflict_when_both_changed() {
        assert!(matches!(
            determine_sync_status(&SyncStatusCheckInput {
                local_head: &Some(changed_head()),
                current_head: &head(),
                remote_head: &Some(changed_head2())
            }),
            CheckSyncResult::Conflict {
                local: _,
                remote: _
            }
        ));
    }
}
