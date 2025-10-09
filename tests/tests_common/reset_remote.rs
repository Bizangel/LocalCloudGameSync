use globset::Glob;
use globset::GlobSetBuilder;
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
    let saves_path = Path::new(REMOTE_TEST_SAVE_PATH).join("GameSaves");
    let snapshots_path = Path::new(REMOTE_TEST_SAVE_PATH).join("Snapshots");
    let cloudmeta_path = Path::new(REMOTE_TEST_SAVE_PATH).join(".cloudmeta");

    if saves_path.exists() {
        fs::remove_dir_all(saves_path)
            .expect("Unable to delete GameSaves path on Post-Test Remote Cleanup");
    }
    if snapshots_path.exists() {
        fs::remove_dir_all(snapshots_path)
            .expect("Unable to delete Snapshots path on Post-Test Remote Cleanup");
    }

    delete_head_files(&cloudmeta_path)
        .expect("Unable to delete HEAD files on Post-Test Remote Cleanup");
}
