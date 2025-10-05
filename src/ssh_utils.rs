use std::{
    path::Path,
    process::{Command, ExitStatus},
};

use crate::config::config_commons::REMOTE_HEAD_FOLDER;

/// Result of an SSH command
#[derive(Debug)]
pub struct SshOutput {
    pub code: ExitStatus,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub input_command: String,
}

impl SshOutput {
    pub fn code_display(&self) -> String {
        return self
            .code
            .code()
            .map(|c| c.to_string())
            .unwrap_or(String::from("<none>"));
    }

    pub fn output_lossy(&self) -> String {
        return format!(
            "{}{}",
            String::from_utf8_lossy(&self.stdout).to_string(),
            String::from_utf8_lossy(&self.stderr).to_string(),
        );
    }

    pub fn print(&self) {
        println!("Input Command: {}", self.input_command);
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
        .map_err(|e| e.to_string())?;

    // println!("Executing: ssh {}", cmd);
    if output.status.code() == Some(255) {
        let error = String::from_utf8(output.stderr).unwrap_or_default();
        return Err(format!("SSH Connection Error:\n{}", error));
    }

    Ok(SshOutput {
        code: output.status,
        stdout: output.stdout,
        stderr: output.stderr,
        input_command: cmd.to_string(),
    })
}

pub fn ssh_restic_backup(
    ssh_host: &str,
    remote_save_folder_path: &str,
    remote_backup_key: &str,
) -> Result<(), String> {
    let exists_command = format!(
        "cd {dir} 2>/dev/null || exit 100; \
        [ ! -r {REMOTE_HEAD_FOLDER}/restic_password ] && exit 99; \
        [ ! -d Snapshots/{key} ] && {{ restic init -r Snapshots/{key} -p {REMOTE_HEAD_FOLDER}/restic_password || exit 98; }}; \
        restic -r Snapshots/test-backup/ -p {REMOTE_HEAD_FOLDER}/restic_password backup RemoteSaves/{key}",
        dir = &remote_save_folder_path,
        key = &remote_backup_key
    );

    let res = ssh_command(&ssh_host, &exists_command)?;

    return match res.code.code() {
        Some(0) => Ok(()),
        Some(99) => Err(format!(
            "{REMOTE_HEAD_FOLDER}/restic_password does not exist or is unreadable!",
        )),
        Some(_) | None => Err(format!(
            "Error ocurred during SSH restic backup calls - Exit Code:{}\n{}",
            res.code_display(),
            res.output_lossy()
        )),
    };
}

pub fn scp_folder(
    ssh_host: &str,
    src_folder: &Path,
    dst_folder: &str,
) -> Result<SshOutput, String> {
    let scp_target = format!("{}:{}", ssh_host, dst_folder);
    let scp_source = src_folder
        .to_str()
        .ok_or_else(|| String::from("Invalid source folder for scp"))?;

    let args = ["-r", scp_source, &scp_target];
    let output = Command::new("scp")
        .args(args)
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
        input_command: format!("scp {}", &args.join(" ")),
    })
}
