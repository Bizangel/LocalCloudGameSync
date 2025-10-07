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
    let mut command = Command::new("ssh");
    command.arg(host).args(["-p", &port.to_string()]);

    #[cfg(feature = "insecure-ssh")]
    {
        command.args([
            "-o",
            "StrictHostKeyChecking=no",
            "-o",
            "UserKnownHostsFile=/dev/null",
        ]);
    }

    command.arg(cmd);

    let output = command.output().map_err(|e| e.to_string())?;
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

/// Internal helper to run `scp` with the given arguments.
fn run_scp(args: &[&str]) -> Result<SshOutput, String> {
    println!("Executing: scp {}", args.join(" "));
    let mut command = Command::new("scp");

    #[cfg(feature = "insecure-ssh")]
    {
        command.args([
            "-o",
            "StrictHostKeyChecking=no",
            "-o",
            "UserKnownHostsFile=/dev/null",
        ]);
    }

    command.args(args);

    let output = command.output().map_err(|e| e.to_string())?;
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

/// Copy from local -> remote via scp
pub fn scp_to_remote(
    ssh_host: &str,
    scp_port: u32,
    src_folder: &Path,
    dst_folder: &str,
) -> Result<SshOutput, String> {
    let scp_source = src_folder
        .to_str()
        .ok_or_else(|| String::from("Invalid source folder for scp"))?;

    let scp_target = format!("{}:{}", ssh_host, dst_folder);

    let args = ["-P", &scp_port.to_string(), "-r", scp_source, &scp_target];
    run_scp(&args)
}

/// Copy from remote -> local via scp
pub fn scp_from_remote(
    ssh_host: &str,
    scp_port: u32,
    src_folder: &str,
    dst_folder: &Path,
) -> Result<SshOutput, String> {
    let scp_source = format!("{}:{}", ssh_host, src_folder);

    let scp_target = dst_folder
        .to_str()
        .ok_or_else(|| String::from("Invalid destination folder for scp"))?;
    // Add separator + dot to ensure folder contents are copied
    let scp_target = format!("{}{}.", scp_target, MAIN_SEPARATOR_STR);

    let args = ["-P", &scp_port.to_string(), "-r", &scp_source, &scp_target];
    run_scp(&args)
}
