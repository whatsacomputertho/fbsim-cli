use clap::{Subcommand, Args};

/// Display a matchup from a playoff round
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonPlayoffsRoundMatchupGetArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The year of the season
    #[arg(short='y')]
    #[arg(long="year")]
    pub year: usize,

    /// The playoff round index
    #[arg(short='r')]
    #[arg(long="round")]
    pub round: usize,

    /// The matchup index in the round
    #[arg(short='m')]
    #[arg(long="matchup")]
    pub matchup: usize,
}

/// Simulate a matchup from a playoff round
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonPlayoffsRoundMatchupSimArgs {
    /// The playback speed
    #[arg(short='s')]
    #[arg(long="speed")]
    pub playback_speed: Option<f64>,

    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The playoff round index
    #[arg(short='r')]
    #[arg(long="round")]
    pub round: usize,

    /// The matchup index in the round
    #[arg(short='m')]
    #[arg(long="matchup")]
    pub matchup: usize,
}

/// Manage matchups for a playoff round
#[derive(Subcommand, Clone)]
pub enum FbsimLeagueSeasonPlayoffsRoundMatchupSubcommand {
    Get(FbsimLeagueSeasonPlayoffsRoundMatchupGetArgs),
    Sim(FbsimLeagueSeasonPlayoffsRoundMatchupSimArgs),
}
