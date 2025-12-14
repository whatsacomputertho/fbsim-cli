use clap::{Args, Subcommand};

/// The sim subcommand arguments
#[derive(Args, Clone)]
pub struct FbsimGameScoreSimArgs {
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
}

/// The benchmark subcommand arguments
#[derive(Args, Clone)]
pub struct FbsimGameScoreBenchmarkArgs {}

/// The fbsim game score subcommands
#[derive(Subcommand, Clone)]
pub enum FbsimGameScoreSubcommand {
    Benchmark(FbsimGameScoreBenchmarkArgs),
    Sim(FbsimGameScoreSimArgs)
}
