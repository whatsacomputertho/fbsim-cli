mod cli;

use crate::cli::{
    FbsimCli,
    FbsimSubcommand,
    FbsimGameSubcommand,
    FbsimGameSimArgs,
    OutputFormat
};

use clap::Parser;
use fbsim_core::sim::BoxScoreSimulator;
use fbsim_core::team::FootballTeam;
use serde_json;
use std::fs;
use std::str::FromStr;

fn simulate_game(args: FbsimGameSimArgs) {
    // Load the home and away teams from their files
    let home_team: FootballTeam = serde_json::from_str(
        &fs::read_to_string(&args.home).unwrap()
    ).unwrap();
    let away_team: FootballTeam = serde_json::from_str(
        &fs::read_to_string(&args.away).unwrap()
    ).unwrap();

    // Instantiate the simulator
    let box_score_sim = BoxScoreSimulator::new();

    // Instantiate an RNG and simulate
    let mut rng = rand::thread_rng();
    let box_score = box_score_sim.sim(
        &home_team,
        &away_team,
        &mut rng
    );

    // Serialize the box score as a string based on the given output format
    let output_format = OutputFormat::from_str(
        &args.output_format.clone().unwrap_or(String::from(""))
    ).unwrap();
    let box_score_str: String = match output_format {
        OutputFormat::Json => {
            serde_json::to_string_pretty(&box_score).unwrap()
        },
        OutputFormat::Default => {
            format!("{}", box_score)
        }
    };

    // Write the box scores either to stdout or to a file
    match &args.output_file {
        Some(x) => {
            // Write the output to the output file
            _ = fs::write(x, box_score_str);
        },
        None => {
            // Print the output to stdout
            println!("{}", box_score_str);
        }
    }
}

fn main() {
    // Parse the command-line args
    let fbdb_cli = FbsimCli::parse();

    // Perform the subcommand
    let command = fbdb_cli.command();
    match &command {
        FbsimSubcommand::Game { command } => match command {
            FbsimGameSubcommand::Sim(args) => simulate_game(args.clone())
        }
    }
}
