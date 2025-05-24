use clap::{Subcommand, Args};

/// Add a new team to the FootballSim league
#[derive(Args, Clone)]
pub struct FbsimLeagueTeamAddArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String
}

/// Create a team for a new FootballSim season
#[derive(Args, Clone)]
pub struct FbsimLeagueTeamCreateArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The ID of the team in the league
    #[arg(short='t')]
    #[arg(long="team")]
    pub id: usize,

    /// The team name
    #[arg(short='n')]
    #[arg(long="name")]
    pub name: String,

    /// The base64 encoded team logo
    #[arg(long="logo")]
    pub logo: String,

    /// The team's offensive overall
    #[arg(long="offense")]
    pub offense: i32,

    /// The team's defensive overall
    #[arg(long="defense")]
    pub defense: i32,

    /// The output filepath for the season team
    #[arg(short='f')]
    #[arg(long="file")]
    pub file: String,
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
    Create(FbsimLeagueTeamCreateArgs),
    Get(FbsimLeagueTeamGetArgs),
    List(FbsimLeagueTeamListArgs)
}
