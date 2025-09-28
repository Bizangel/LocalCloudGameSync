use clap::{Parser, Subcommand};

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
}

fn main() {
    let args = LocalGameSyncCli::parse();

    match args.command {
        Commands::Sync { save_key } => local_cloud_game_sync::sync_command::sync(&save_key),
        Commands::Dryrun => println!("Dryrun isn't implemented yet!"),
    }
}
