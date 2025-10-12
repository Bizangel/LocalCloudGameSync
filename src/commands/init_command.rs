use crate::config::{default_sync_config_path, init_default_config};

pub fn init_command() -> Result<(), String> {
    let initted_folder = init_default_config()?;
    println!(
        "Initialized global config at: {} - Please ensure to fill out it's values",
        default_sync_config_path()?.display()
    );

    println!("Initialized config folders at {}", initted_folder.display());
    Ok(())
}
