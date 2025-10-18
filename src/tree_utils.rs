use globset::GlobSet;
use md5;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const CHECKSUM_BUFFER_MB: usize = 5;

/// Computes a deterministic hash by calculating the md5 of each 5mb chunks of file
fn digest_file(path: &Path) -> io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = vec![0u8; CHECKSUM_BUFFER_MB * 1024 * 1024];
    let mut chunk_digests: Vec<u8> = Vec::new();

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        let digest = md5::compute(&buffer[..n]);
        chunk_digests.extend_from_slice(&digest.0); // append raw 16 bytes
    }

    // Hash the concatenated digests to get a final checksum
    let final_digest = md5::compute(&chunk_digests);
    Ok(format!("{:x}", final_digest))
}

/// Recursively walk a folder, calling `callback` for each file (not for dirs).
fn walk_folder_rec<F>(
    root: &Path,
    path: &Path,
    ignore_globset: &GlobSet,
    callback: &mut F,
) -> Result<(), String>
where
    F: FnMut(&Path, &Path) -> Result<(), String>,
{
    for entry in fs::read_dir(path).map_err(|e| format!("Unable to read directory\n{}", e))? {
        let entry = entry.map_err(|e| format!("Error listing directory entry: {}", e))?;
        let path = entry.path();
        // Ignore symlinks
        if fs::symlink_metadata(&path)
            .map_err(|e| {
                format!(
                    "Unable to read symlink metadata for {}\n{}",
                    path.display(),
                    e
                )
            })?
            .file_type()
            .is_symlink()
        {
            continue;
        }

        // Skip ignored paths
        if ignore_globset.is_match(&path) {
            continue;
        }

        if path.is_dir() {
            // recurse into subdir
            walk_folder_rec(&root, &path, ignore_globset, callback)?;
        } else if path.is_file() {
            let rel = path
                .strip_prefix(root)
                .map_err(|e| format!("Error processing file {}\n{}", path.display(), e))?;
            callback(&path, &rel)?;
        }
    }

    Ok(())
}

fn walk_folder<F>(path: &Path, ignore_globset: &GlobSet, callback: &mut F) -> Result<(), String>
where
    F: FnMut(&Path, &Path) -> Result<(), String>,
{
    return walk_folder_rec(path, path, ignore_globset, callback);
}

// TODO: Make multi-threaded for faster checksumming - usually fine for save folders
/// Recursively compute the MD5 checksum of a folder
/// Also returns the last modification timestamp of all files (max modified time).
pub fn tree_folder_hash(path: &Path, ignore_globset: &GlobSet) -> Result<(String, u64), String> {
    let mut entries: Vec<(String, String)> = Vec::new();
    let mut latest_mod_time: SystemTime = UNIX_EPOCH;

    walk_folder(path, ignore_globset, &mut |filepath, rel_path| {
        // Compute hash
        let file_md5 = digest_file(filepath)
            .map_err(|e| format!("Error checksumming file {}\n{}", filepath.display(), e))?;
        entries.push((rel_path.to_string_lossy().to_string(), file_md5));

        // Update last modified timestamp
        let metadata = filepath
            .metadata()
            .map_err(|e| format!("Unable to read metadata for {}\n{}", filepath.display(), e))?;
        if let Ok(modified) = metadata.modified() {
            if modified > latest_mod_time {
                latest_mod_time = modified;
            }
        }

        Ok(())
    })?;

    // Sort entries by name for deterministic hash
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let combined = entries
        .iter()
        .map(|(name, hash)| format!("{}:{}", name, hash))
        .collect::<Vec<_>>()
        .join("\n");

    let folder_hash = format!("{:x}", md5::compute(combined.as_bytes()));

    // Convert latest_mod_time to UNIX timestamp (seconds)
    let latest_mod_unix = latest_mod_time
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("SystemTime before UNIX_EPOCH: {}", e))?
        .as_secs();

    Ok((folder_hash, latest_mod_unix))
}

fn get_tmp_sync_directory() -> PathBuf {
    return env::temp_dir().join("local_cloud_game_sync_tmp");
}

pub struct UploadTempFolder {
    pub path: PathBuf,
}

impl Drop for UploadTempFolder {
    fn drop(&mut self) {
        let _ = delete_tmp_sync_directory();
    }
}

// TODO: Make multi-threaded for faster checksumming - usually fine for save folders
pub fn tree_folder_temp_copy(
    path: &Path,
    ignore_globset: &GlobSet,
) -> Result<UploadTempFolder, String> {
    let target = get_tmp_sync_directory();
    delete_tmp_sync_directory()?;

    walk_folder(path, ignore_globset, &mut |filepath, relpath| {
        let target = target.join(relpath);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Error creating parent dir {}: {}", parent.display(), e))?;
        }
        fs::copy(filepath, &target).map_err(|e| {
            format!(
                "Error copying {} -> {}\n{}",
                filepath.display(),
                target.display(),
                e
            )
        })?;

        Ok(())
    })?;

    Ok(UploadTempFolder { path: target })
}

pub fn delete_tmp_sync_directory() -> Result<(), String> {
    let tmpdir = get_tmp_sync_directory();
    if !tmpdir.exists() {
        return Ok(());
    }

    fs::remove_dir_all(tmpdir).map_err(|e| format!("Unable to delete directory\n{}", e))?;
    Ok(())
}
