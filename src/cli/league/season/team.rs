pub mod stats;

use clap::{Subcommand, Args};

use crate::cli::league::season::team::stats::FbsimLeagueSeasonTeamStatsSubcommand;

/// Add a team to a new FootballSim season
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonTeamAddArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The input filepath for the team
    #[arg(short='t')]
    #[arg(long="team")]
    pub team: String,

    /// The ID of the team in the league
    #[arg(short='i')]
    #[arg(long="id")]
    pub id: usize
}

/// Display a team from a FootballSim season
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonTeamGetArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The ID of the team in the league
    #[arg(short='t')]
    #[arg(long="team")]
    pub id: usize,

    /// The year of the season
    #[arg(short='y')]
    #[arg(long="year")]
    pub year: usize,

    /// Number of playoff teams (for playoff picture calculation)
    #[arg(short='n')]
    #[arg(long="num-playoff-teams")]
    #[arg(default_value="4")]
    pub num_playoff_teams: usize
}

/// List all teams from a FootballSim season
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonTeamListArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The year of the season
    #[arg(short='y')]
    #[arg(long="year")]
    pub year: usize,

    /// Number of playoff teams (for playoff picture calculation)
    #[arg(short='n')]
    #[arg(long="num-playoff-teams")]
    #[arg(default_value="4")]
    pub num_playoff_teams: usize
}

/// Manage teams for a season of a FootballSim league
#[derive(Subcommand, Clone)]
pub enum FbsimLeagueSeasonTeamSubcommand {
    Add(FbsimLeagueSeasonTeamAddArgs),
    Get(FbsimLeagueSeasonTeamGetArgs),
    List(FbsimLeagueSeasonTeamListArgs),
    Stats {
        #[command(subcommand)]
        command: FbsimLeagueSeasonTeamStatsSubcommand
    }
}
