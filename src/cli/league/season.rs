use clap::{Subcommand, Args};

/// Create a new season for the FootballSim league
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonCreateArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The input directory path for the league season teams
    #[arg(short='t')]
    #[arg(long="teams")]
    pub teams: String,
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
    Create(FbsimLeagueSeasonCreateArgs),
    Get(FbsimLeagueSeasonGetArgs),
    List(FbsimLeagueSeasonListArgs)
}
