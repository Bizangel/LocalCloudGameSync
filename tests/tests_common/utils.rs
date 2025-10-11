use local_cloud_game_sync::config::config_commons::REMOTE_HEAD_FOLDER_NAME;
use local_cloud_game_sync::config::config_commons::REMOTE_SNAPSHOT_FOLDER_NAME;

use crate::tests_common::common::REMOTE_TEST_SAVE_PATH;
use crate::tests_common::common::TEMP_RESTIC_RESTORE_PATH;
use crate::tests_common::restic_helper::ResticSnapshotManifest;
use crate::tests_common::test_local_folder::TestTempFolder;
use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn restic_snapshots_cmd_call(repo_path: &Path, password_file: &Path) -> io::Result<String> {
    // Sanity checks
    if !repo_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Repository path not found",
        ));
    }
    if !password_file.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Password file not found",
        ));
    }

    // Run restic
    let output = Command::new("restic")
        .arg("-r")
        .arg(repo_path)
        .arg("--password-file")
        .arg(password_file)
        .arg("snapshots")
        .arg("--json")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("restic failed: {}", stderr),
        ));
    }

    // Return JSON output
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout)
}

pub fn restic_restore_cmd_call(
    repo_path: &Path,
    password_file: &Path,
    snapshot_id: &str,
    target: &Path,
) -> io::Result<()> {
    // Sanity checks
    if !repo_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Repository path not found",
        ));
    }
    if !password_file.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Password file not found",
        ));
    }

    let output = Command::new("restic")
        .arg("-r")
        .arg(repo_path)
        .arg("--password-file")
        .arg(password_file)
        .arg("restore")
        .arg(snapshot_id)
        .arg("--target")
        .arg(target)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("restic restore failed: {}", stderr),
        ));
    }

    Ok(())
}

pub fn get_remote_restic_snapshots(sync_key: &str) -> io::Result<Vec<ResticSnapshotManifest>> {
    let repo_location = Path::new(REMOTE_TEST_SAVE_PATH)
        .join(REMOTE_SNAPSHOT_FOLDER_NAME)
        .join(sync_key);
    let cloudmeta_path = Path::new(REMOTE_TEST_SAVE_PATH)
        .join(REMOTE_HEAD_FOLDER_NAME)
        .join("restic_password");

    let calljson = restic_snapshots_cmd_call(&repo_location, &cloudmeta_path)?;
    let parse: Vec<ResticSnapshotManifest> = serde_json::from_str(&calljson)?;

    Ok(parse)
}

pub fn restore_restic_snapshot(sync_key: &str, snapshot_id: &str) -> io::Result<TestTempFolder> {
    let repo_location = Path::new(REMOTE_TEST_SAVE_PATH)
        .join(REMOTE_SNAPSHOT_FOLDER_NAME)
        .join(sync_key);
    let cloudmeta_path = Path::new(REMOTE_TEST_SAVE_PATH)
        .join(REMOTE_HEAD_FOLDER_NAME)
        .join("restic_password");

    let restored_path = Path::new(TEMP_RESTIC_RESTORE_PATH);
    restic_restore_cmd_call(&repo_location, &cloudmeta_path, snapshot_id, restored_path)?;

    Ok(TestTempFolder::from_path(restored_path))
}
