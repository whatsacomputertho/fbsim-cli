use clap::{Parser, Subcommand, Args};
use std::str::FromStr;

/// fbsim command-line interface
///
/// Defines the command-line interface for the fbsim CLI
#[derive(Parser)]
#[command(name="Football simulation command-line interface")]
#[command(author="whatsacomputertho")]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
pub struct FbsimCli {
    /// The subcommand passed in via the CLI
    #[command(subcommand)]
    pub command: FbsimSubcommand
}

impl FbsimCli {
    pub fn command(&self) -> FbsimSubcommand {
        self.command.clone()
    }
}

/// The subcommands of the fbsim CLI
#[derive(Subcommand, Clone)]
pub enum FbsimSubcommand {
    Game {
        #[command(subcommand)]
        command: FbsimGameSubcommand
    }
}

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

/// The subcommands of the fbsim game command
#[derive(Subcommand, Clone)]
pub enum FbsimGameSubcommand {
    Sim(FbsimGameSimArgs)
}

/// Enum into which the output format argument is parsed
#[derive(Debug,PartialEq)]
pub enum OutputFormat {
    Json,
    Default
}
impl FromStr for OutputFormat {
    type Err = ();
    fn from_str(input: &str) -> Result<OutputFormat, Self::Err> {
        match input {
            "json"      => Ok(OutputFormat::Json),
            _           => Ok(OutputFormat::Default),
        }
    }
}
impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt_str = match self {
            OutputFormat::Json => "json",
            OutputFormat::Default => "default"
        };
        f.write_str(&fmt_str)
    }
}
