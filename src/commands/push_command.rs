use crate::config::load_and_validate_config;
use crate::remote_lock::RemoteLock;
use crate::ssh_utils::ssh_command;

pub fn push_command(save_config_key: &String) -> Result<(), String> {
    let config = load_and_validate_config(save_config_key)?;

    // 1. Get remote lock
    let _lock = RemoteLock::acquire(&config.ssh_host)?;

    // 2. Check remote head
    let check_command = format!(
        "cd {} && test -f .cloudmeta/{}",
        &config.remote_save_folder_path, save_config_key
    );

    println!("{}", check_command);
    // let res = ssh_command(&global_save_options.ssh_host, &check_command)?;
    let res = ssh_command(&config.ssh_host, &check_command)?;

    res.print();

    Ok(())
}
