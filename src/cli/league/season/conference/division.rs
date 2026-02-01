use clap::{Subcommand, Args};

/// Add a division to a conference
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonConferenceDivisionAddArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The conference index
    #[arg(short='c')]
    #[arg(long="conference")]
    pub conference: usize,

    /// The name of the division
    #[arg(short='n')]
    #[arg(long="name")]
    pub name: String,
}

/// List divisions in a conference
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonConferenceDivisionListArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The year of the season
    #[arg(short='y')]
    #[arg(long="year")]
    pub year: usize,

    /// The conference index
    #[arg(short='c')]
    #[arg(long="conference")]
    pub conference: usize,
}

/// Get a division from a conference
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonConferenceDivisionGetArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The year of the season
    #[arg(short='y')]
    #[arg(long="year")]
    pub year: usize,

    /// The conference index
    #[arg(short='c')]
    #[arg(long="conference")]
    pub conference: usize,

    /// The division ID
    #[arg(short='d')]
    #[arg(long="division")]
    pub division: usize,
}

/// Manage divisions in a conference
#[derive(Subcommand, Clone)]
pub enum FbsimLeagueSeasonConferenceDivisionSubcommand {
    Add(FbsimLeagueSeasonConferenceDivisionAddArgs),
    List(FbsimLeagueSeasonConferenceDivisionListArgs),
    Get(FbsimLeagueSeasonConferenceDivisionGetArgs),
}
