pub mod score;
pub mod play;

use clap::Subcommand;

use crate::cli::game::play::FbsimGamePlaySubcommand;
use crate::cli::game::score::FbsimGameScoreSubcommand;

/// The subcommands of the fbsim game command
#[derive(Subcommand, Clone)]
pub enum FbsimGameSubcommand {
    Play {
        #[command(subcommand)]
        command: FbsimGamePlaySubcommand
    },
    Score {
        #[command(subcommand)]
        command: FbsimGameScoreSubcommand
    }
}
