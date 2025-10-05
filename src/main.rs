use clap::{Parser, Subcommand};
use local_cloud_game_sync::commands;

const RED_ANSI_ESCAPE: &str = "\x1b[31m";
const ANSI_RESET_ESCAPE: &str = "\x1b[0m";

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

    /// Perform uni-directional pull process for the given game key. Pulls the remote version overwriting the local folder.
    /// Pull can be a destructive action - hence it is recommended to ensure that your current version is already on the cloud.
    /// NOTE: PULL does not respect filter configs. If one accidentally uploaded a should-be-ignored file - it will be included if it is on the remote.
    Pull {
        save_key: String,

        /// Helper scripting option to only perform the pull operation if the observed remote head is the one provided.
        /// Allows to perform atomical operations.
        #[arg(long)]
        if_head: Option<String>,
    },

    /// Perform uni-directional push process for the given game key. Uploads the current local save overwriting the remote.
    /// Because push can be a destructive action - a snapshot is always triggered on the remote before overwriting.
    Push {
        save_key: String,
        /// Helper scripting option to only perform the push operation if the observed remote head is the one provided.
        /// Allows to perform atomical operations.
        #[arg(long)]
        if_head: Option<String>,
    },
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
        Commands::Push { save_key, if_head } => {
            commands::push_command(&save_key, if_head.as_deref())
        }
        Commands::Pull { save_key, if_head } => {
            commands::pull_command(&save_key, if_head.as_deref())
        }
        Commands::InitConfig => commands::init_command(),
        Commands::OpenConfigFolder => commands::open_config_folder_command(),
        Commands::Dryrun => Err(String::from("Dryrun isn't implemented yet!")),
    };

    if let Err(e) = command_res {
        eprintln!("{RED_ANSI_ESCAPE}{e}{ANSI_RESET_ESCAPE}");
    }
}
