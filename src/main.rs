use std::{path::PathBuf, process::ExitCode};

use clap::{Parser, Subcommand};
use local_cloud_game_sync::{commands, config::config_commons::load_config};

const RED_ANSI_ESCAPE: &str = "\x1b[31m";
const ANSI_RESET_ESCAPE: &str = "\x1b[0m";

/// Bidirectional syncing program using SSH and rsync whilst also backing up game saves.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct LocalGameSyncCli {
    // Sync command to execute
    #[command(subcommand)]
    command: Commands,

    /// An optional global config file override - uses default global config location if not specified.
    #[arg(long)]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Perform the remote check to see what sync actions need to be performed.
    /// Determines whether local be fast-forwarded - or remote can be fast-forwarded - or whether there's a conflict that requires manual approval.
    CheckSync {
        sync_key: String,

        /// Displays status short and concise - useful for scripting.
        #[arg(long)]
        short: bool,
    },

    /// Perform uni-directional pull process for the given game key. Pulls the remote version overwriting the local folder.
    /// Pull can be a destructive action - hence it is recommended to ensure that your current version is already on the cloud.
    /// NOTE: PULL does not respect filter configs. If one accidentally uploaded a should-be-ignored file - it will be included if it is on the remote.
    Pull {
        sync_key: String,

        /// Helper scripting option to only perform the pull operation if the observed remote head is the one provided.
        /// Allows to perform atomical operations.
        #[arg(long)]
        if_head: Option<String>,
    },

    /// Perform uni-directional push process for the given game key. Uploads the current local save overwriting the remote.
    /// Because push can be a destructive action - a snapshot is always triggered on the remote before overwriting.
    Push {
        sync_key: String,
        /// Helper scripting option to only perform the push operation if the observed remote head is the one provided.
        /// Allows to perform atomical operations.
        #[arg(long)]
        if_head: Option<String>,
    },
    /// Shows what CLI would do without actually performing the sync for the given game key settings.
    Dryrun,
    /// Opens the default config file
    OpenConfig,
    /// Ensures that the configs folder exists to start placing save sync configurations.
    InitConfig,
}

fn handle_command(args: LocalGameSyncCli) -> Result<(), String> {
    let command_res: Result<(), String> = match args.command {
        Commands::CheckSync { sync_key, short } => {
            let sync_config = load_config(&sync_key, args.config.as_deref())?;
            commands::check_sync_command(&sync_config, short).map(|_| ())
        }
        Commands::Push { sync_key, if_head } => {
            let sync_config = load_config(&sync_key, args.config.as_deref())?;
            commands::push_command(&sync_config, if_head.as_deref())
        }
        Commands::Pull { sync_key, if_head } => {
            let sync_config = load_config(&sync_key, args.config.as_deref())?;
            commands::pull_command(&sync_config, if_head.as_deref())
        }
        Commands::InitConfig => commands::init_command(),
        Commands::OpenConfig => commands::open_default_config_file(),
        Commands::Dryrun => Err(String::from("Dryrun isn't implemented yet!")),
    };

    return command_res;
}

fn main() -> ExitCode {
    let args = LocalGameSyncCli::parse();

    let command_res = handle_command(args);
    if let Err(e) = command_res {
        eprintln!("{RED_ANSI_ESCAPE}{e}{ANSI_RESET_ESCAPE}");
        return ExitCode::FAILURE;
    }

    return ExitCode::SUCCESS;
}
