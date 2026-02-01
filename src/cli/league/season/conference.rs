pub mod division;

use clap::{Subcommand, Args};

use crate::cli::league::season::conference::division::FbsimLeagueSeasonConferenceDivisionSubcommand;

/// Add a conference to the current season
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonConferenceAddArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The name of the conference
    #[arg(short='n')]
    #[arg(long="name")]
    pub name: String,
}

/// List conferences in a season
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonConferenceListArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The year of the season
    #[arg(short='y')]
    #[arg(long="year")]
    pub year: usize,
}

/// Get a conference from a season
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonConferenceGetArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The year of the season
    #[arg(short='y')]
    #[arg(long="year")]
    pub year: usize,

    /// The conference ID
    #[arg(short='c')]
    #[arg(long="conference")]
    pub conference: usize,
}

/// Manage conferences for a season
#[derive(Subcommand, Clone)]
pub enum FbsimLeagueSeasonConferenceSubcommand {
    Add(FbsimLeagueSeasonConferenceAddArgs),
    List(FbsimLeagueSeasonConferenceListArgs),
    Get(FbsimLeagueSeasonConferenceGetArgs),
    Division {
        #[command(subcommand)]
        command: FbsimLeagueSeasonConferenceDivisionSubcommand
    }
}
