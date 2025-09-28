use std::io;
use std::process::{Command, ExitStatus};

/// Result of an SSH command
pub struct SshOutput {
    pub code: ExitStatus,
    pub stdout: Vec<u8>,
}

/// Runs a command over SSH and returns both the ExitStatus and stdout
pub fn ssh_command(host: &str, cmd: &str) -> io::Result<SshOutput> {
    let output = Command::new("ssh").arg(host).arg(cmd).output()?; // capture stdout and stderr

    Ok(SshOutput {
        code: output.status,
        stdout: output.stdout,
    })
}
