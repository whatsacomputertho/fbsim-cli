use clap::{Subcommand, Args};

/// Get the passing stats for each team in the league
#[derive(Args, Clone)]
pub struct FbsimLeagueTeamStatsPassingArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String
}

/// Get the rushing stats for each team in the league
#[derive(Args, Clone)]
pub struct FbsimLeagueTeamStatsRushingArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String
}

/// Get the receiving stats for each team in the league
#[derive(Args, Clone)]
pub struct FbsimLeagueTeamStatsReceivingArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String
}

/// Manage teams for an existing FootballSim league
#[derive(Subcommand, Clone)]
pub enum FbsimLeagueTeamStatsSubcommand {
    Passing(FbsimLeagueTeamStatsPassingArgs),
    Rushing(FbsimLeagueTeamStatsRushingArgs),
    Receiving(FbsimLeagueTeamStatsReceivingArgs)
}
