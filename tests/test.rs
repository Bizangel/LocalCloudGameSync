mod tests_common;

use local_cloud_game_sync::commands::push_command;
use local_cloud_game_sync::config::LocalSaveOptionsJson;

use crate::tests_common::temp_global_config::TempGlobalConfig;
use crate::tests_common::temp_local_config::TempLocalConfig;
use crate::tests_common::test_local_folder::TestLocalFolder;

#[test]
pub fn mytest() {
    let testfolder = TestLocalFolder::with_test_folder();
    let globalcfg = TempGlobalConfig::get_global_config();
    let _cfg = TempLocalConfig::with_config(&LocalSaveOptionsJson {
        remote_sync_key: "testKey".to_string(),
        save_folder_path: testfolder.path.to_str().unwrap().to_string(),
        save_ignore_glob: [].to_vec(),
    });

    push_command(&_cfg.config_key, None, Some(&globalcfg.override_path)).expect("Failed to push");
}
