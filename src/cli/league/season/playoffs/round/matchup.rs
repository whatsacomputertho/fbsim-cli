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

    /// The conference bracket index (optional, for conference playoffs)
    #[arg(short='c')]
    #[arg(long="conference")]
    pub conference: Option<usize>,

    /// Get a matchup from the winners bracket instead of a conference bracket
    #[arg(short='w')]
    #[arg(long="winners-bracket")]
    pub winners_bracket: bool,
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

    /// The conference bracket index (defaults to 0)
    #[arg(short='c')]
    #[arg(long="conference")]
    #[arg(default_value_t = 0)]
    pub conference: usize,

    /// Simulate a matchup from the winners bracket instead of a conference bracket
    #[arg(short='w')]
    #[arg(long="winners-bracket")]
    pub winners_bracket: bool,
}

/// Manage matchups for a playoff round
#[derive(Subcommand, Clone)]
pub enum FbsimLeagueSeasonPlayoffsRoundMatchupSubcommand {
    Get(FbsimLeagueSeasonPlayoffsRoundMatchupGetArgs),
    Sim(FbsimLeagueSeasonPlayoffsRoundMatchupSimArgs),
}
