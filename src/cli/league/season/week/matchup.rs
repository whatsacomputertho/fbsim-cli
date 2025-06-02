use clap::{Subcommand, Args};

/// Display a matchup from a week of a FootballSim season
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonWeekMatchupGetArgs {
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
    pub week: usize,

    /// The ID of the matchup in the week
    #[arg(short='m')]
    #[arg(long="matchup")]
    pub matchup: usize,
}

/// Simulate a matchup from a week of a FootballSim season
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonWeekMatchupSimArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The ID of the week in the season
    #[arg(short='w')]
    #[arg(long="week")]
    pub week: usize,

    /// The ID of the matchup in the week
    #[arg(short='m')]
    #[arg(long="matchup")]
    pub matchup: usize,
}

/// Manage matchups for a week of a FootballSim season
#[derive(Subcommand, Clone)]
pub enum FbsimLeagueSeasonWeekMatchupSubcommand {
    Get(FbsimLeagueSeasonWeekMatchupGetArgs),
    Sim(FbsimLeagueSeasonWeekMatchupSimArgs)
}
