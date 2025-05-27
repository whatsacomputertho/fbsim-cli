pub mod team;
pub mod season;

use clap::{Subcommand, Args};

use crate::cli::league::team::FbsimLeagueTeamSubcommand;
use crate::cli::league::season::FbsimLeagueSeasonSubcommand;

/// Create a new FootballSim league
#[derive(Args, Clone)]
pub struct FbsimLeagueCreateArgs {
    /// The file to write to
    #[arg(short='f')]
    #[arg(long="file")]
    pub output_file: String,
}

/// Manage FootballSim leagues
#[derive(Subcommand, Clone)]
pub enum FbsimLeagueSubcommand {
    Create(FbsimLeagueCreateArgs),
    Team {
        #[command(subcommand)]
        command: FbsimLeagueTeamSubcommand
    },
    Season {
        #[command(subcommand)]
        command: FbsimLeagueSeasonSubcommand
    }
}
