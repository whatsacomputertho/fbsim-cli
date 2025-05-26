pub mod team;

use clap::{Subcommand, Args};

use crate::cli::league::season::team::FbsimLeagueSeasonTeamSubcommand;

/// Add a new season to the FootballSim league
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonAddArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String
}

/// Get a past or current season for the FootballSim league
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonGetArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The year of the season to display
    #[arg(short='y')]
    #[arg(long="year")]
    pub year: usize
}

/// List all past and current seasons for the FootballSim league
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonListArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String
}

/// Manage seasons for an existing FootballSim league
#[derive(Subcommand, Clone)]
pub enum FbsimLeagueSeasonSubcommand {
    Add(FbsimLeagueSeasonAddArgs),
    Get(FbsimLeagueSeasonGetArgs),
    List(FbsimLeagueSeasonListArgs),
    Team {
        #[command(subcommand)]
        command: FbsimLeagueSeasonTeamSubcommand
    },
}
