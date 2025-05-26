use clap::{Subcommand, Args};

/// Add a team to a new FootballSim season
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonTeamAddArgs {
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
    pub offense: usize,

    /// The team's defensive overall
    #[arg(long="defense")]
    pub defense: usize,
}

/// Manage teams for a season of a FootballSim league
#[derive(Subcommand, Clone)]
pub enum FbsimLeagueSeasonTeamSubcommand {
    Add(FbsimLeagueSeasonTeamAddArgs)
}
