use clap::{Subcommand, Args};

/// Get the passing stats for each team in the season
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonTeamStatsPassingArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The year of the season
    #[arg(short='y')]
    #[arg(long="year")]
    pub year: usize
}

/// Get the rushing stats for each team in the season
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonTeamStatsRushingArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The year of the season
    #[arg(short='y')]
    #[arg(long="year")]
    pub year: usize
}

/// Get the receiving stats for each team in the season
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonTeamStatsReceivingArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The year of the season
    #[arg(short='y')]
    #[arg(long="year")]
    pub year: usize
}

/// Manage teams for an existing FootballSim league season
#[derive(Subcommand, Clone)]
pub enum FbsimLeagueSeasonTeamStatsSubcommand {
    Passing(FbsimLeagueSeasonTeamStatsPassingArgs),
    Rushing(FbsimLeagueSeasonTeamStatsRushingArgs),
    Receiving(FbsimLeagueSeasonTeamStatsReceivingArgs)
}
