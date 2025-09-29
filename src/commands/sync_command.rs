use crate::local_save_config::get_config;

pub fn sync_command(save_key: &String) -> Result<(), String> {
    let config = get_config(save_key)?;

    let Some(config) = config else {
        println!("Configuration not found for key {}", save_key);
        return Ok(());
    };

    println!("{:#?}", config);

    Ok(())
}
