use std::fs;
use std::str::FromStr;

use rand::Rng;

use fbsim_core::game::play::DriveSimulator;
use fbsim_core::game::context::{GameContext, GameContextBuilder};
use fbsim_core::team::FootballTeam;

use crate::cli::game::drive::FbsimGameDriveSimArgs;
use crate::cli::output::OutputFormat;

use serde_json;

pub fn drive_sim(args: FbsimGameDriveSimArgs) -> Result<(), String> {
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

    // Decide whether to update the context
    let is_context_given: bool = match &args.context {
        Some(_) => true,
        None => false
    };
    let update_context: bool = match &args.update_context {
        Some(x) => {
            if is_context_given {
                *x
            } else {
                return Err(format!("Cannot update context, no context given"));
            }
        },
        None => false
    };

    // Load the context from its file or initialize
    let mut rng = rand::thread_rng();
    let context: GameContext = if is_context_given {
        match &args.context {
            Some(x) => {
                let context_file_res = &fs::read_to_string(x);
                let context_file = match context_file_res {
                    Ok(f) => f,
                    Err(e) => return Err(format!("Error loading context file: {}", e))
                };
                match serde_json::from_str(context_file) {
                    Ok(c) => c,
                    Err(e) => return Err(format!("Error loading context from file: {}", e))
                }
            },
            None => return Err(String::from("Unreachable error"))
        }
    } else {
        let home_opening_kickoff: bool = rng.gen::<bool>();
        GameContextBuilder::new()
            .home_team_short(home_team.short_name())
            .away_team_short(away_team.short_name())
            .home_possession(!home_opening_kickoff)
            .home_positive_direction(!home_opening_kickoff)
            .home_opening_kickoff(home_opening_kickoff)
            .build()
            .unwrap()
    };

    // Check if the game is over
    if context.game_over() {
        return Err(String::from("Cannot simulate drive, game is already over"));
    }

    // Instantiate the simulator and simulate the drive
    let drive_sim = DriveSimulator::new();
    let (drive, new_context) = drive_sim.sim(
        &home_team,
        &away_team,
        context,
        &mut rng
    );

    // Serialize the drive result as a string based on the given output format
    let output_format = OutputFormat::from_str(
        &args.output_format.clone().unwrap_or(String::from(""))
    ).unwrap();
    let drive_str: String = match output_format {
        OutputFormat::Json => {
            serde_json::to_string_pretty(&drive).unwrap()
        },
        OutputFormat::Default => {
            format!("{}", drive)
        }
    };

    // If the context should be updated, update it
    if update_context {
        match &args.context {
            Some(x) => {
                let context_str = match serde_json::to_string_pretty(&new_context) {
                    Ok(s) => s,
                    Err(e) => return Err(format!("Error updating context: {}", e))
                };
                _ = fs::write(x, context_str)
            },
            None => return Err(String::from("Unreachable error"))
        };
    }

    // Write the drive result either to stdout or to a file
    match &args.output_file {
        Some(x) => {
            // Write the output to the output file
            _ = fs::write(x, drive_str);
        },
        None => {
            // Print the output to stdout
            println!("{}", drive_str);
        }
    };
    Ok(())
}
