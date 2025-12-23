use clap::{Args, Subcommand};

/// The sim subcommand arguments
#[derive(Args, Clone)]
pub struct FbsimGameDriveSimArgs {
    /// The format to output
    #[arg(short='o')]
    #[arg(long="output")]
    pub output_format: Option<String>,

    /// The file to write to
    #[arg(short='f')]
    #[arg(long="file")]
    pub output_file: Option<String>,

    /// A path to a file specifying the current game context
    #[arg(long="context")]
    pub context: Option<String>,

    /// Whether to overwrite the context with the new context
    #[arg(long="update-context")]
    pub update_context: Option<bool>,

    /// A path to a file specifying the game's home team
    #[arg(long="home")]
    pub home: String,

    /// A path to a file specifying the game's away team
    #[arg(long="away")]
    pub away: String,
}

/// The fbsim game drive subcommands
#[derive(Subcommand, Clone)]
pub enum FbsimGameDriveSubcommand {
    Sim(FbsimGameDriveSimArgs)
}
