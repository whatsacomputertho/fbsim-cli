pub mod matchup;

use clap::{Subcommand, Args};

use crate::cli::league::season::playoffs::round::matchup::FbsimLeagueSeasonPlayoffsRoundMatchupSubcommand;

/// Display a playoff round
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonPlayoffsRoundGetArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The year of the season
    #[arg(short='y')]
    #[arg(long="year")]
    pub year: usize,

    /// The playoff round ID
    #[arg(short='r')]
    #[arg(long="round")]
    pub round: usize,

    /// Get conference-specific round (optional, for multi-conference playoffs)
    #[arg(short='c')]
    #[arg(long="conference")]
    pub conference: Option<usize>,

    /// Get winners bracket round (optional, for multi-conference playoffs)
    #[arg(short='w')]
    #[arg(long="winners-bracket")]
    pub winners_bracket: bool,
}

/// Simulate a playoff round
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonPlayoffsRoundSimArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,
}

/// Manage rounds in the playoffs
#[derive(Subcommand, Clone)]
pub enum FbsimLeagueSeasonPlayoffsRoundSubcommand {
    Get(FbsimLeagueSeasonPlayoffsRoundGetArgs),
    Sim(FbsimLeagueSeasonPlayoffsRoundSimArgs),
    Matchup {
        #[command(subcommand)]
        command: FbsimLeagueSeasonPlayoffsRoundMatchupSubcommand
    }
}
