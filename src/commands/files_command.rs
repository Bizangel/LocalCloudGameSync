use crate::{config::RuntimeSyncConfig, tree_utils::collect_matching_files};

const RED_ANSI_ESCAPE: &str = "\x1b[31m";
const MAGENTA_ANSI_ESCAPE: &str = "\x1b[36m";
const ANSI_RESET_ESCAPE: &str = "\x1b[0m";

pub fn files_command(sync_config: &RuntimeSyncConfig) -> Result<(), String> {
    let (tracked_files, ignored_entries) =
        collect_matching_files(&sync_config.local_save_folder, &sync_config.ignore_globset)?;

    println!(
        "{MAGENTA_ANSI_ESCAPE}Sync key:{ANSI_RESET_ESCAPE} {}",
        sync_config.remote_sync_key
    );
    println!(
        "{MAGENTA_ANSI_ESCAPE}Save Folder: {ANSI_RESET_ESCAPE} {}",
        sync_config.local_save_folder.display()
    );
    println!("{MAGENTA_ANSI_ESCAPE}Tracked Files: {ANSI_RESET_ESCAPE}");
    if tracked_files.is_empty() {
        println!("<no files>");
    } else {
        for entry in &tracked_files {
            println!("\t{entry}");
        }
    }

    println!("{RED_ANSI_ESCAPE}Ignored files:{ANSI_RESET_ESCAPE}");
    if ignored_entries.is_empty() {
        println!("<no entries>");
    } else {
        for entry in &ignored_entries {
            println!("\t{entry}");
        }
    }

    Ok(())
}
