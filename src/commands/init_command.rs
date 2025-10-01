use crate::local_save_config::{get_global_sync_config_path, init_configs_folder};

pub fn init_command() -> Result<(), String> {
    let initted_folder = init_configs_folder()?;
    println!(
        "Initialized global config at: {} - Please ensure to fill out it's values",
        get_global_sync_config_path()?.display()
    );
    println!("Initialized config folders at {}", initted_folder.display());
    Ok(())
}
