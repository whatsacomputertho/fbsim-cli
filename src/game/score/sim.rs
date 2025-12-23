use std::fs;
use std::str::FromStr;

use fbsim_core::game::score::FinalScoreSimulator;
use fbsim_core::team::FootballTeam;

use crate::cli::game::score::FbsimGameScoreSimArgs;
use crate::cli::output::OutputFormat;

use serde_json;

pub fn final_score_sim(args: FbsimGameScoreSimArgs) -> Result<(), String> {
    // Load the home and away teams from their files
    let home_team_file_res = &fs::read_to_string(&args.home);
    let home_team_file = match home_team_file_res {
        Ok(file) => file,
        Err(e) => return Err(format!("Error loading home team file: {}", e)),
    };
    let home_team: FootballTeam = match serde_json::from_str(home_team_file) {
        Ok(team) => team,
        Err(e) => return Err(format!("Error loading home team: {}", e)),
    };
    let away_team_file_res = &fs::read_to_string(&args.away);
    let away_team_file = match away_team_file_res {
        Ok(file) => file,
        Err(e) => return Err(format!("Error loading away team file: {}", e)),
    };
    let away_team: FootballTeam = match serde_json::from_str(away_team_file) {
        Ok(team) => team,
        Err(e) => return Err(format!("Error loading away team: {}", e)),
    };

    // Instantiate the simulator and simulate
    let final_score_sim = FinalScoreSimulator::new();
    let mut rng = rand::thread_rng();
    let score = match final_score_sim.sim(&home_team, &away_team, &mut rng) {
        Ok(s) => s,
        Err(e) => return Err(format!("Error generating final score: {}", e))
    };

    // Serialize the final score as a string based on the given output format
    let output_format = OutputFormat::from_str(
        &args.output_format.clone().unwrap_or(String::from(""))
    ).unwrap();
    let score_str: String = match output_format {
        OutputFormat::Json => {
            serde_json::to_string_pretty(&score).unwrap()
        },
        OutputFormat::Default => {
            format!("{}", score)
        }
    };

    // Write the final scores either to stdout or to a file
    match &args.output_file {
        Some(x) => {
            // Write the output to the output file
            _ = fs::write(x, score_str);
        },
        None => {
            // Print the output to stdout
            println!("{}", score_str);
        }
    };
    Ok(())
}
