pub mod team;

use clap::{Subcommand, Args};

use crate::cli::league::team::FbsimLeagueTeamSubcommand;

/// Create a new FootballSim league
#[derive(Args, Clone)]
pub struct FbsimLeagueCreateArgs {
    /// The format to output
    #[arg(short='o')]
    #[arg(long="output")]
    pub output_format: Option<String>,

    /// The file to write to
    #[arg(short='f')]
    #[arg(long="file")]
    pub output_file: Option<String>,
}

/// Manage FootballSim leagues
#[derive(Subcommand, Clone)]
pub enum FbsimLeagueSubcommand {
    Create(FbsimLeagueCreateArgs),
    Team {
        #[command(subcommand)]
        command: FbsimLeagueTeamSubcommand
    }
}
