mod tests_common;

use local_cloud_game_sync::commands::{CheckSyncResult, check_sync_command, push_command};
use local_cloud_game_sync::config::LocalSaveOptionsJson;

use crate::tests_common::reset_remote::reset_remote_repository;
use crate::tests_common::temp_global_config::TempGlobalConfig;
use crate::tests_common::temp_local_config::TempLocalConfig;
use crate::tests_common::test_local_folder::TestLocalFolder;
use crate::tests_common::utils::get_remote_restic_snapshots;

#[test]
pub fn mytest() {
    reset_remote_repository();

    let testfolder = TestLocalFolder::with_test_folder();
    let globalcfg = TempGlobalConfig::get_global_config();
    let _cfg = TempLocalConfig::with_config(&LocalSaveOptionsJson {
        remote_sync_key: "testKey".to_string(),
        save_folder_path: testfolder.path.to_str().unwrap().to_string(),
        save_ignore_glob: [].to_vec(),
    });

    let sync_result =
        check_sync_command(&_cfg.config_key, false, Some(&globalcfg.override_path)).unwrap();
    assert_eq!(sync_result, CheckSyncResult::RemoteEmpty);

    push_command(&_cfg.config_key, None, Some(&globalcfg.override_path)).expect("Failed to push");

    let snapshots = get_remote_restic_snapshots("testKey").expect("success");
    assert_eq!(snapshots.len(), 1); // There should be a single one new snapshot

    // validate snapshot restore.

    println!("{:#?}", snapshots);
}
