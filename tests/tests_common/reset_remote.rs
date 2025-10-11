use globset::Glob;
use globset::GlobSetBuilder;
use local_cloud_game_sync::config::config_commons::REMOTE_HEAD_FOLDER_NAME;
use local_cloud_game_sync::config::config_commons::REMOTE_SAVES_FOLDER_NAME;
use local_cloud_game_sync::config::config_commons::REMOTE_SNAPSHOT_FOLDER_NAME;
use std::fs;
use std::io;
use std::path::Path;

use crate::tests_common::common::REMOTE_TEST_SAVE_PATH;

pub fn delete_head_files(dir: &Path) -> io::Result<()> {
    // Build a glob matcher for "*.HEAD"
    let mut builder = GlobSetBuilder::new();

    builder.add(Glob::new("*.HEAD").unwrap());
    let matcher = builder.build().unwrap();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                if matcher.is_match(file_name) {
                    fs::remove_file(&path)?;
                }
            }
        }
    }

    Ok(())
}

pub fn reset_remote_repository() {
    println!("Resetting remote!");
    let saves_path = Path::new(REMOTE_TEST_SAVE_PATH).join(REMOTE_SAVES_FOLDER_NAME);
    let snapshots_path = Path::new(REMOTE_TEST_SAVE_PATH).join(REMOTE_SNAPSHOT_FOLDER_NAME);
    let cloudmeta_path = Path::new(REMOTE_TEST_SAVE_PATH).join(REMOTE_HEAD_FOLDER_NAME);

    if saves_path.exists() {
        fs::remove_dir_all(saves_path)
            .expect("Unable to delete game saves path on Post-Test Remote Cleanup");
    }
    if snapshots_path.exists() {
        fs::remove_dir_all(snapshots_path)
            .expect("Unable to delete snapshots path on Post-Test Remote Cleanup");
    }

    delete_head_files(&cloudmeta_path)
        .expect("Unable to delete HEAD files on Post-Test Remote Cleanup");
}
