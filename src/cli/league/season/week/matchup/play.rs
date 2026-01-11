use clap::{Subcommand, Args};

/// Simulate the next play for a matchup of a FootballSim season
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonWeekMatchupPlaySimArgs {
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

/// Manage plays for a matchup of a FootballSim season
#[derive(Subcommand, Clone)]
pub enum FbsimLeagueSeasonWeekMatchupPlaySubcommand {
    Sim(FbsimLeagueSeasonWeekMatchupPlaySimArgs)
}
