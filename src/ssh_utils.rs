use std::process::{Command, ExitStatus};

/// Result of an SSH command
#[derive(Debug)]
pub struct SshOutput {
    pub code: ExitStatus,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl SshOutput {
    pub fn print(&self) {
        println!("Exit Code: {}", self.code);
        println!(
            "Stdout: {}",
            String::from_utf8_lossy(&self.stdout).to_string()
        );
        println!(
            "Stderr: {}",
            String::from_utf8_lossy(&self.stderr).to_string()
        );
    }
}

/// Runs a command over SSH and returns both the ExitStatus and stdout
pub fn ssh_command(host: &str, cmd: &str) -> Result<SshOutput, String> {
    let output = Command::new("ssh")
        .arg(host)
        .arg(cmd)
        .output()
        .map_err(|e| e.to_string())?; // capture stdout and stderr

    if output.status.code() == Some(255) {
        let error = String::from_utf8(output.stderr).unwrap_or_default();
        return Err(format!("SSH Connection Error:\n{}", error));
    }

    Ok(SshOutput {
        code: output.status,
        stdout: output.stdout,
        stderr: output.stderr,
    })
}

pub fn ssh_cat_head(
    ssh_host: &str,
    remote_save_folder_path: &str,
    remote_backup_key: &str,
) -> Result<Option<String>, String> {
    let exists_command = format!(
        "cd {dir} 2>/dev/null || exit 100; \
        [ -r .cloudmeta/{key}.HEAD ] && cat .cloudmeta/{key}.HEAD && exit 0; \
        [ -e .cloudmeta/{key}.HEAD ] && exit 1; \
        exit 2",
        dir = &remote_save_folder_path,
        key = &remote_backup_key
    );

    let res = ssh_command(&ssh_host, &exists_command)?;
    return match res.code.code() {
        Some(0) => String::from_utf8(res.stdout)
            .map(|x| Some(String::from(x.trim())))
            .map_err(|e| format!("Unable to read file HEAD {}", e)),
        Some(1) => Err(String::from("Remote HEAD file is not readable")),
        Some(2) => Ok(None),
        Some(_) | None => Err(format!(
            "Error ocurred during checking SSH remote HEAD - Exit Code: \n{}",
            String::from_utf8_lossy(&res.stderr)
        )),
    };
}
