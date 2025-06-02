use clap::{Subcommand, Args};

/// Add a new team to the FootballSim league
#[derive(Args, Clone)]
pub struct FbsimLeagueTeamAddArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String
}

/// Display historical information about a team in the league
#[derive(Args, Clone)]
pub struct FbsimLeagueTeamGetArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The ID of the team to display
    #[arg(short='t')]
    #[arg(long="team")]
    pub team: usize
}

/// List all teams in the league
#[derive(Args, Clone)]
pub struct FbsimLeagueTeamListArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String
}

/// Manage teams for an existing FootballSim league
#[derive(Subcommand, Clone)]
pub enum FbsimLeagueTeamSubcommand {
    Add(FbsimLeagueTeamAddArgs),
    Get(FbsimLeagueTeamGetArgs),
    List(FbsimLeagueTeamListArgs)
}
