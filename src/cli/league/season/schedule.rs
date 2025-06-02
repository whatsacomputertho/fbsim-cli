use clap::{Subcommand, Args};

/// Generate a schedule for the current season of a FootballSim league
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonScheduleGenArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String
}

/// Manage the schedule for the current season of a FootballSim league
#[derive(Subcommand, Clone)]
pub enum FbsimLeagueSeasonScheduleSubcommand {
    Gen(FbsimLeagueSeasonScheduleGenArgs)
}
