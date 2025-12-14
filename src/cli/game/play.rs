use clap::{Args, Subcommand};

/// The sim subcommand arguments
#[derive(Args, Clone)]
pub struct FbsimGamePlaySimArgs {
    /// The format to output
    #[arg(short='o')]
    #[arg(long="output")]
    pub output_format: Option<String>,

    /// The file to write to
    #[arg(short='f')]
    #[arg(long="file")]
    pub output_file: Option<String>,

    /// A path to a file specifying the game's home team
    #[arg(long="home")]
    pub home: String,

    /// A path to a file specifying the game's away team
    #[arg(long="away")]
    pub away: String,

    /// A path to a file specifying the current game context
    #[arg(long="context")]
    pub context: String,
}

/// The fbsim game play subcommands
#[derive(Subcommand, Clone)]
pub enum FbsimGamePlaySubcommand {
    Sim(FbsimGamePlaySimArgs)
}
