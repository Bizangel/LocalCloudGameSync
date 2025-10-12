use globset::Glob;
use globset::GlobSetBuilder;

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

pub fn delete_all_head_files(dir: &Path) -> io::Result<()> {
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
