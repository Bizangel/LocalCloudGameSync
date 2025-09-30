use globset::{Glob, GlobSetBuilder};

use crate::{local_save_config::get_config, remote_lock::RemoteLock, tree_hash};
use std::path;

pub fn sync_command(save_key: &String) -> Result<(), String> {
    let config = get_config(save_key)?;
    let Some(config) = config else {
        println!("Configuration not found for key {}", save_key);
        return Ok(());
    };

    let save_folder_path = path::Path::new(&config.save_folder_path);
    if !save_folder_path.exists() {
        println!(
            "Given save folder path {} does not exist - unable to sync",
            config.save_folder_path
        );
        return Ok(());
    }

    // 2. Hash folder
    let mut builder = GlobSetBuilder::new();
    for pat in config.save_ignore_glob {
        let pattern = Glob::new(&pat).map_err(|e| format!("Invalid glob pattern: {}", e))?;
        builder.add(pattern);
    }
    let ignore_globset = builder
        .build()
        .map_err(|e| format!("Unable to build globset\n{}", e))?;

    let res = tree_hash::tree_folder_hash(save_folder_path, &ignore_globset)?;
    println!("checksum {}", res);
    // 1. Acquire remote lock.
    // let _lock = RemoteLock::acquire(&config.ssh_host)
    //     .map_err(|e| format!("Unable to get remote lock:\n{}", e))?;

    // if !_lock.is_acquired() {
    //     println!("Unable to obtain remote lock - stopping sync");
    //     return Ok(());
    // }

    // println!("{:#?}", config);

    Ok(())
}
