pub mod round;

use clap::{Subcommand, Args};

use crate::cli::league::season::playoffs::round::FbsimLeagueSeasonPlayoffsRoundSubcommand;

/// Generate playoffs for the current season
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonPlayoffsGenArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The number of teams in the playoffs
    #[arg(short='n')]
    #[arg(long="num-teams")]
    pub num_teams: usize,
}

/// Display the playoffs for a season
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonPlayoffsGetArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The year of the season
    #[arg(short='y')]
    #[arg(long="year")]
    pub year: usize,
}

/// Simulate the entire playoffs
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonPlayoffsSimArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,
}

/// Manage playoffs for a season
#[derive(Subcommand, Clone)]
pub enum FbsimLeagueSeasonPlayoffsSubcommand {
    Gen(FbsimLeagueSeasonPlayoffsGenArgs),
    Get(FbsimLeagueSeasonPlayoffsGetArgs),
    Sim(FbsimLeagueSeasonPlayoffsSimArgs),
    Round {
        #[command(subcommand)]
        command: FbsimLeagueSeasonPlayoffsRoundSubcommand
    }
}
