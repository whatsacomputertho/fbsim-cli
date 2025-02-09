mod cli;

use crate::cli::{
    FbsimCli,
    FbsimSubcommand,
    FbsimGameSubcommand,
    FbsimGameSimArgs,
    FbsimGameBenchmarkArgs,
    OutputFormat
};

use clap::Parser;
use fbsim_core::sim::BoxScoreSimulator;
use fbsim_core::team::FootballTeam;
use indicatif::ProgressBar;
use serde_json;
use std::fs;
use std::io::{Write, stdout};
use std::str::FromStr;
use tabwriter::TabWriter;

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
    ).unwrap();

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

fn game_benchmark(_args: FbsimGameBenchmarkArgs) {
    // Instantiate the simulator and RNG
    let box_score_sim = BoxScoreSimulator::new();
    let mut rng = rand::thread_rng();

    // Instantiate 2D arrays to store the win and tie propotions
    let mut win_props = [[0.0_f64; 11]; 11];
    let mut tie_props = [[0.0_f64; 11]; 11];

    // Instantiate a progress bar for benchmark progress
    let progress_bar = ProgressBar::new(11 * 11 * 500);

    // Run many game simulations and track the observed
    // win and tie proportions by skill differential
    for i in 0..11 {
        // Set the home offense and away defense
        let home_off = ((10 - i) * 10) as i32;
        let away_def = (i * 10) as i32;
        for j in 0..11 {
            // Set the away offense and home defense
            let away_off = ((10 - j) * 10) as i32;
            let home_def = (j * 10) as i32;
            for k in 1..501 {
                // Create the home and away teams from their files
                let home_team = FootballTeam::from_properties(
                    "Home Team",
                    home_off,
                    home_def
                ).unwrap();
                let away_team = FootballTeam::from_properties(
                    "Away Team",
                    away_off,
                    away_def
                ).unwrap();

                // Simulate the game
                let box_score = box_score_sim.sim(
                    &home_team,
                    &away_team,
                    &mut rng
                ).unwrap();

                // Decide whether this was a tie, or home win / away win
                let was_tie = box_score.home_score() == box_score.away_score();
                let home_win = box_score.home_score() > box_score.away_score();
                let away_win = box_score.home_score() < box_score.away_score();

                // Increment the tie proportions
                tie_props[i][j] = if was_tie {
                    ((tie_props[i][j] * ((k as f64) - 1_f64)) + 1_f64) / (k as f64)
                } else {
                    ((tie_props[i][j] * ((k as f64) - 1_f64)) + 0_f64) / (k as f64)
                };
                tie_props[j][i] = tie_props[i][j];

                // Increment the win proportions
                win_props[i][j] = if home_win {
                    ((win_props[i][j] * ((k as f64) - 1_f64) + 1_f64)) / (k as f64)
                } else {
                    ((win_props[i][j] * ((k as f64) - 1_f64) + 0_f64)) / (k as f64)
                };
                win_props[j][i] = if away_win {
                    ((win_props[j][i] * ((k as f64) - 1_f64) + 1_f64)) / (k as f64)
                } else {
                    ((win_props[j][i] * ((k as f64) - 1_f64) + 0_f64)) / (k as f64)
                };

                // Increment the progress bar
                progress_bar.inc(1);
            }
        }
    }

    // Display the win probability table
    println!("");
    let mut tw = TabWriter::new(stdout());
    let mut win_table_lines = String::from(
        "\t0\t\t\t\t\t\t\t\t\t\t100"
    );
    for i in 0..11 {
        let table_line = win_props[i].iter().map(|x| format!("{:.2}", x)).collect::<Vec<_>>().join("\t");
        let table_line_pfx = if i == 0 || i == 10 {
            format!("{}", i * 10)
        } else {
            String::from("")
        };
        win_table_lines = win_table_lines + "\n" + &table_line_pfx + "\t" + &table_line;
    }
    println!("");
    println!("Win probabilities:");
    write!(&mut tw, "{}", &win_table_lines).unwrap();
    tw.flush().unwrap();

    // Display the tie probability table
    println!("");
    let mut tie_table_lines = String::from(
        "\t0\t\t\t\t\t\t\t\t\t\t100"
    );
    for i in 0..11 {
        let table_line = tie_props[i].iter().map(|x| format!("{:.2}", x)).collect::<Vec<_>>().join("\t");
        let table_line_pfx = if i == 0 || i == 10 {
            format!("{}", i * 10)
        } else {
            String::from("")
        };
        tie_table_lines = tie_table_lines + "\n" + &table_line_pfx + "\t" + &table_line;
    }
    println!("");
    println!("Tie probabilities:");
    write!(&mut tw, "{}", &tie_table_lines).unwrap();
    tw.flush().unwrap();
    println!("");
}

fn main() {
    // Parse the command-line args
    let fbdb_cli = FbsimCli::parse();

    // Perform the subcommand
    let command = fbdb_cli.command();
    match &command {
        FbsimSubcommand::Game { command } => match command {
            FbsimGameSubcommand::Sim(args) => simulate_game(args.clone()),
            FbsimGameSubcommand::Benchmark(args) => game_benchmark(args.clone())
        }
    }
}
