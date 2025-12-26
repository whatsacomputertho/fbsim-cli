pub mod drive;
pub mod score;
pub mod play;

use clap::{Args, Subcommand};

use crate::cli::game::drive::FbsimGameDriveSubcommand;
use crate::cli::game::play::FbsimGamePlaySubcommand;
use crate::cli::game::score::FbsimGameScoreSubcommand;

/// The sim subcommand arguments
#[derive(Args, Clone)]
pub struct FbsimGameSimArgs {
    /// The playback speed
    #[arg(short='s')]
    #[arg(long="speed")]
    pub playback_speed: Option<f64>,

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
    Benchmark(FbsimGameBenchmarkArgs),
    Sim(FbsimGameSimArgs),
    Play {
        #[command(subcommand)]
        command: FbsimGamePlaySubcommand
    },
    Drive {
        #[command(subcommand)]
        command: FbsimGameDriveSubcommand
    },
    Score {
        #[command(subcommand)]
        command: FbsimGameScoreSubcommand
    }
}
