use clap::{Subcommand, Args};

/// The sim subcommand arguments
#[derive(Args, Clone)]
pub struct FbsimGameSimArgs {
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
pub struct FbsimGameBenchmarkArgs {}

/// The subcommands of the fbsim game command
#[derive(Subcommand, Clone)]
pub enum FbsimGameSubcommand {
    Sim(FbsimGameSimArgs),
    Benchmark(FbsimGameBenchmarkArgs)
}
