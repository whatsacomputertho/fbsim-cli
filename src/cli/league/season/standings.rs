use clap::Args;

/// Display standings for a season
#[derive(Args, Clone)]
pub struct FbsimLeagueSeasonStandingsArgs {
    /// The input filepath for the league
    #[arg(short='l')]
    #[arg(long="league")]
    pub league: String,

    /// The year of the season
    #[arg(short='y')]
    #[arg(long="year")]
    pub year: usize,

    /// Filter by conference index (optional)
    #[arg(short='c')]
    #[arg(long="conference")]
    pub conference: Option<usize>,

    /// Filter by division ID (requires -c/--conference)
    #[arg(short='d')]
    #[arg(long="division")]
    pub division: Option<usize>,

    /// Group standings by conference
    #[arg(long="by-conference")]
    pub by_conference: bool,

    /// Group standings by division
    #[arg(long="by-division")]
    pub by_division: bool,
}
