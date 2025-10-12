use std::{
    fs, io,
    path::{Path, PathBuf},
};

use globset::GlobSet;
use local_cloud_game_sync::{
    common::Revision,
    config::config_commons::{
        REMOTE_HEAD_FOLDER_NAME, REMOTE_SAVES_FOLDER_NAME, REMOTE_SNAPSHOT_FOLDER_NAME,
    },
    tree_utils::tree_folder_hash,
};

use crate::tests_common::{
    common::{REMOTE_TEST_ROOT_PATH, TESTING_RESOURCES_ROOT},
    restic_helper::ResticSnapshotManifest,
    temp_folder::TestTempFolder,
    utils::{delete_all_head_files, restic_restore_cmd_call, restic_snapshots_cmd_call},
};

pub struct TestRemote {
    remote_snapshots_folder_path: PathBuf,
    remote_heads_folder_path: PathBuf,
    remote_saves_folder_path: PathBuf,
}

impl TestRemote {
    pub fn builder() -> TestRemoteBuilder {
        TestRemoteBuilder::new()
    }

    pub fn read_remote_head(&self, sync_key: &str) -> Result<Option<Revision>, String> {
        let remote_head_path = self
            .remote_heads_folder_path
            .join(format!("{}.HEAD", sync_key));

        if !remote_head_path.exists() {
            return Ok(None);
        }

        let folderbytes = fs::read(remote_head_path)
            .map_err(|e| format!("Unable to read remote head hash\n{e}"))?;

        let headstr = String::from_utf8(folderbytes)
            .map_err(|e| format!("Invalid UTF8 bytes reading remote head hash\n{e}"))?;

        let rev = Revision::deserialize(&headstr)?;
        Ok(Some(rev))
    }

    pub fn get_snapshots(&self, sync_key: &str) -> io::Result<Vec<ResticSnapshotManifest>> {
        let repo_location = self.remote_snapshots_folder_path.join(sync_key);

        let calljson =
            restic_snapshots_cmd_call(&repo_location, &self._get_restic_password_file())?;
        let parse: Vec<ResticSnapshotManifest> = serde_json::from_str(&calljson)?;

        Ok(parse)
    }

    pub fn restore_restic_snapshot(
        &self,
        sync_key: &str,
        snapshot_id: &str,
    ) -> io::Result<TestTempFolder> {
        let repo_location = self.remote_snapshots_folder_path.join(sync_key);
        let restored_path = Path::new(TESTING_RESOURCES_ROOT)
            .join("temp_restic_snapshots")
            .join(snapshot_id);

        restic_restore_cmd_call(
            &repo_location,
            &self._get_restic_password_file(),
            snapshot_id,
            &restored_path,
        )?;

        Ok(TestTempFolder::from_path(restored_path))
    }

    pub fn get_remote_hash(&self, sync_key: &str) -> String {
        tree_folder_hash(
            &self.remote_saves_folder_path.join(sync_key),
            &GlobSet::empty(),
        )
        .unwrap()
    }

    pub fn reset_remote(&mut self) {
        println!("Resetting remote!");
        if self.remote_saves_folder_path.exists() {
            fs::remove_dir_all(&self.remote_saves_folder_path)
                .expect("Unable to delete game saves path on Post-Test Remote Cleanup");
        }

        if self.remote_snapshots_folder_path.exists() {
            fs::remove_dir_all(&self.remote_snapshots_folder_path)
                .expect("Unable to delete snapshots path on Post-Test Remote Cleanup");
        }

        delete_all_head_files(&self.remote_heads_folder_path)
            .expect("Unable to delete HEAD files on Post-Test Remote Cleanup");
    }

    fn _get_restic_password_file(&self) -> PathBuf {
        return self.remote_heads_folder_path.join("restic_password");
    }
}

pub struct TestRemoteBuilder {
    empty_remote: bool,
}

impl TestRemoteBuilder {
    pub fn new() -> Self {
        Self {
            empty_remote: false,
        }
    }

    pub fn with_empty_remote(mut self) -> Self {
        self.empty_remote = true;
        self
    }

    pub fn build(self) -> TestRemote {
        let snapshots_folder = Path::new(REMOTE_TEST_ROOT_PATH).join(REMOTE_SNAPSHOT_FOLDER_NAME);
        let heads_folder = Path::new(REMOTE_TEST_ROOT_PATH).join(REMOTE_HEAD_FOLDER_NAME);
        let saves_folder = Path::new(REMOTE_TEST_ROOT_PATH).join(REMOTE_SAVES_FOLDER_NAME);

        let mut test_remote = TestRemote {
            remote_snapshots_folder_path: snapshots_folder.to_path_buf(),
            remote_heads_folder_path: heads_folder.to_path_buf(),
            remote_saves_folder_path: saves_folder.to_path_buf(),
        };

        if self.empty_remote {
            test_remote.reset_remote();
        }

        return test_remote;
    }
}
