use globset::GlobSet;
use md5;
use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};

const CHECKSUM_BUFFER_MB: usize = 5;

/// Computes a deterministic hash by calculating the md5 of each 5mb chunks of file
pub fn digest_file(path: &PathBuf) -> io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = vec![0u8; CHECKSUM_BUFFER_MB * 1024 * 1024]; // 5 MB buffer
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

// TODO: Make multi-threaded for faster checksumming - usually fine for save folders
/// Recursively compute the MD5 checksum of a folder
pub fn tree_folder_hash(path: &Path, ignore_globset: &GlobSet) -> Result<String, String> {
    let mut entries: Vec<(String, String)> = Vec::new();

    for entry in fs::read_dir(path).map_err(|e| format!("Unable to read directory\n{}", e))? {
        let entry = entry.map_err(|e| format!("Error listing save folder files {}", e))?;
        let path = entry.path();
        let file_name = entry.file_name().into_string().map_err(|e| {
            format!(
                "Unsupported non unicode filename in save folder\n{}",
                e.display()
            )
        })?;

        // Ignore symlinks
        if fs::symlink_metadata(&path)
            .map_err(|e| {
                format!(
                    "Unable to read symlink metadata for file {}\n{}",
                    path.display(),
                    e
                )
            })?
            .file_type()
            .is_symlink()
        {
            continue;
        }

        if ignore_globset.is_match(&path) {
            println!("Ignoring path: {}", path.display());
            continue;
        }

        if path.is_dir() {
            let subfolder_md5 = tree_folder_hash(&path, ignore_globset)?;
            entries.push((file_name, subfolder_md5));
        } else if path.is_file() {
            let file_md5 = digest_file(&path)
                .map_err(|e| format!("Error checksumming file {}\n{}", path.display(), e))?;
            entries.push((file_name, file_md5));
        }
    }

    entries.sort_by(|a, b| a.0.cmp(&b.0));

    // Concatenate "name:md5" strings
    let combined = entries
        .iter()
        .map(|(name, hash)| format!("{}:{}", name, hash))
        .collect::<Vec<String>>()
        .join("\n");

    // Compute MD5 of the combined string for this folder
    Ok(format!("{:x}", md5::compute(combined.as_bytes())))
}

// pub fn tree_folder_hash(path: &Path) -> Result<String, String> {
//     let mut entries: Vec<(String, String)> = Vec::new();

//     for entry in fs::read_dir(path).map_err(|e| format!("Unable to read directory\n{}", e))? {
//         let entry = entry.map_err(|e| format!("Error listing save folder files {}", e))?;
//         let path = entry.path();
//         let file_name = entry.file_name().into_string().map_err(|e| {
//             format!(
//                 "Unsupported non unicode filename in save folder\n{}",
//                 e.display()
//             )
//         })?;

//         // Ignore symlinks
//         if fs::symlink_metadata(&path)
//             .map_err(|e| {
//                 format!(
//                     "Unable to read symlink metadata for file {}\n{}",
//                     path.display(),
//                     e
//                 )
//             })?
//             .file_type()
//             .is_symlink()
//         {
//             continue;
//         }

//         if path.is_dir() {
//             let subfolder_md5 = tree_folder_hash(&path)?;
//             entries.push((file_name, subfolder_md5));
//         } else if path.is_file() {
//             let file_md5 = digest_file(&path)
//                 .map_err(|e| format!("Error checksumming file {}\n{}", path.display(), e))?;
//             entries.push((file_name, file_md5));
//         }
//     }

//     entries.sort_by(|a, b| a.0.cmp(&b.0));

//     // Concatenate "name:md5" strings
//     let combined = entries
//         .iter()
//         .map(|(name, hash)| format!("{}:{}", name, hash))
//         .collect::<Vec<String>>()
//         .join("\n");

//     // Compute MD5 of the combined string for this folder
//     Ok(format!("{:x}", md5::compute(combined.as_bytes())))
// }
