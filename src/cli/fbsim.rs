use clap::{Parser, Subcommand};

use crate::cli::game::FbsimGameSubcommand;
use crate::cli::league::FbsimLeagueSubcommand;

/// fbsim command-line interface
///
/// Defines the command-line interface for the fbsim CLI
#[derive(Parser)]
#[command(name="Football simulation command-line interface")]
#[command(author="whatsacomputertho")]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
pub struct FbsimCli {
    /// The subcommand passed in via the CLI
    #[command(subcommand)]
    pub command: FbsimSubcommand
}

impl FbsimCli {
    pub fn command(&self) -> FbsimSubcommand {
        self.command.clone()
    }
}

/// The subcommands of the fbsim CLI
#[derive(Subcommand, Clone)]
pub enum FbsimSubcommand {
    Game {
        #[command(subcommand)]
        command: FbsimGameSubcommand
    },
    League {
        #[command(subcommand)]
        command: FbsimLeagueSubcommand
    }
}
