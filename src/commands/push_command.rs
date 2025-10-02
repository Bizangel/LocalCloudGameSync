use crate::config::load_and_validate_config;
use crate::remote_lock::RemoteLock;
use crate::ssh_utils::ssh_command;

pub fn push_command(save_config_key: &String) -> Result<(), String> {
    let config = load_and_validate_config(save_config_key)?;

    // 1. Get remote lock
    let _lock = RemoteLock::acquire(&config.ssh_host)?;

    let exists_command = format!(
        "cd {dir} 2>/dev/null || exit 100; \
        [ -r .cloudmeta/{key}.HEAD ] && cat .cloudmeta/{key}.HEAD && exit 0; \
        [ -e .cloudmeta/{key}.HEAD ] && exit 1; \
        exit 2",
        dir = &config.remote_save_folder_path,
        key = &config.remote_backup_key
    );

    let res = ssh_command(&config.ssh_host, &exists_command)?;
    let head_output: Option<String> = match res.code.code() {
        Some(0) => String::from_utf8(res.stdout)
            .map(|x| Some(String::from(x.trim())))
            .map_err(|e| format!("Unable to read file HEAD {}", e)),
        Some(1) => Err(String::from("Remote HEAD file is not readable")),
        Some(2) => Ok(None),
        Some(_) | None => Err(format!(
            "Error ocurred during checking SSH remote HEAD - Exit Code: \n{}",
            String::from_utf8_lossy(&res.stderr)
        )),
    }?;

    println!("{:#?}", head_output);
    // res.print();

    Ok(())
}
