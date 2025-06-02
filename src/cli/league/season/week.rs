pub mod matchup;

use clap::{Subcommand, Args};

use crate::cli::league::season::week::matchup::FbsimLeagueSeasonWeekMatchupSubcommand;

/// Display a week from a FootballSim season
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonWeekGetArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The year of the season
    #[arg(short='y')]
    #[arg(long="year")]
    pub year: usize,

    /// The ID of the week in the season
    #[arg(short='w')]
    #[arg(long="week")]
    pub week: usize
}

/// Simulate a week of a FootballSim season
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonWeekSimArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The ID of the week in the season
    #[arg(short='w')]
    #[arg(long="week")]
    pub week: usize
}

/// List all weeks from a FootballSim season
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonWeekListArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The year of the season
    #[arg(short='y')]
    #[arg(long="year")]
    pub year: usize
}

/// Manage weeks for a season of a FootballSim league
#[derive(Subcommand, Clone)]
pub enum FbsimLeagueSeasonWeekSubcommand {
    Get(FbsimLeagueSeasonWeekGetArgs),
    List(FbsimLeagueSeasonWeekListArgs),
    Sim(FbsimLeagueSeasonWeekSimArgs),
    Matchup {
        #[command(subcommand)]
        command: FbsimLeagueSeasonWeekMatchupSubcommand
    }
}
