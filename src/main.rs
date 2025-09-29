use clap::{Parser, Subcommand};
use local_cloud_game_sync::commands;

/// Bidirectional syncing program using SSH and rsync whilst also backing up game saves.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct LocalGameSyncCli {
    /// Name of the person to greet
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Perform the bi-directional sync process following the given game key settings.
    Sync { save_key: String },
    /// Shows what CLI would do without actually performing the sync for the given game key settings.
    Dryrun,
    /// Opens the config folder in the file explorer
    OpenConfigFolder,
    /// Ensures that the configs folder exists to start placing save sync configurations.
    InitConfig,
}

fn main() {
    let args = LocalGameSyncCli::parse();

    let command_res: Result<(), String> = match args.command {
        Commands::Sync { save_key } => commands::sync_command(&save_key),
        Commands::InitConfig => commands::init_command(),
        Commands::OpenConfigFolder => commands::open_config_folder_command(),
        Commands::Dryrun => Err(String::from("Dryrun isn't implemented yet!")),
    };

    if let Err(e) = command_res {
        eprintln!("Error executing command:");
        eprintln!("{}", e);
    }
}
