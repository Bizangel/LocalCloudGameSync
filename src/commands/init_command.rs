use crate::local_save_config::init_configs_folder;

pub fn init_command() -> Result<(), String> {
    let initted_folder = init_configs_folder()?;
    println!("Initialized config folders at {}", initted_folder.display());
    Ok(())
}
