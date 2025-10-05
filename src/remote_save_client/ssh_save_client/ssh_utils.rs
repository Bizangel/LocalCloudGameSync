use std::{
    path::{MAIN_SEPARATOR_STR, Path},
    process::{Command, ExitStatus},
};

/// Result of an SSH command
#[derive(Debug)]
pub struct SshOutput {
    pub code: ExitStatus,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
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

    #[cfg(debug_assertions)]
    #[allow(dead_code)]
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
pub fn ssh_command(host: &str, port: u32, cmd: &str) -> Result<SshOutput, String> {
    let output = Command::new("ssh")
        .arg(host)
        .args(["-p", port.to_string().as_str()])
        .arg(cmd)
        .output()
        .map_err(|e| e.to_string())?;

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

pub fn scp_to_remote(
    ssh_host: &str,
    scp_port: u32,
    src_folder: &Path,
    dst_folder: &str,
) -> Result<SshOutput, String> {
    let scp_target = format!("{}:{}", ssh_host, dst_folder);
    let scp_source = src_folder
        .to_str()
        .ok_or_else(|| String::from("Invalid source folder for scp"))?;

    let args = ["-P", &scp_port.to_string(), "-r", scp_source, &scp_target];
    println!("Executing: scp {}", args.join(" "));
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
    })
}

pub fn scp_from_remote(
    ssh_host: &str,
    scp_port: u32,
    src_folder: &str,
    dst_folder: &Path,
) -> Result<SshOutput, String> {
    let scp_source = format!("{}:{}", ssh_host, src_folder);
    let scp_target = dst_folder
        .to_str()
        .ok_or_else(|| String::from("Invalid destinations folder for scp"))?;
    let scp_target = format!("{}{}.", scp_target.to_owned(), MAIN_SEPARATOR_STR); // add separator and dot at end - for ensuring that folder contents are copied

    let args = ["-P", &scp_port.to_string(), "-r", &scp_source, &scp_target];
    println!("Executing: scp {}", args.join(" "));
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
    })
}
