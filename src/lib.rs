pub mod commands;
pub mod local_save_config;
pub mod remote_lock;
pub mod ssh_utils;
pub mod utils;
// mod local_save_config;

// pub mod local_save_config;

// use crate::local_save_config;

// use remote_lock::RemoteLock;
// use ssh_utils::ssh_command;

// pub fn main() -> io::Result<()> {
//     let host = "arcanzu-miniserver";

//     // Attempt to acquire lock
//     let lock = RemoteLock::acquire(host)?;

//     if !lock.is_acquired() {
//         println!("Could not acquire lock, exiting.");
//         return Ok(());
//     }

//     println!("Lock acquired, running long-running operation...");

//     // Simulate long-running operation (30 seconds)
//     let long_cmd = "echo 'Starting operation'; sleep 5; echo 'Done'";
//     let status = ssh_command(host, long_cmd)?;

//     println!(
//         "Operation finished with exit code: {:?}",
//         status.code.code()
//     );

//     // Lock will automatically be released here when `lock` goes out of scope
//     Ok(())
// }
