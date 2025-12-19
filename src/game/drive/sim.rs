use std::fs;

use fbsim_core::game::play::DriveSimulator;
use fbsim_core::game::context::GameContext;
use fbsim_core::team::FootballTeam;

use crate::cli::game::drive::FbsimGameDriveSimArgs;

use serde_json;

pub fn drive_sim(args: FbsimGameDriveSimArgs) {
    // Load the home and away teams from their files
    let home_team: FootballTeam = serde_json::from_str(
        &fs::read_to_string(&args.home).unwrap()
    ).unwrap();
    let away_team: FootballTeam = serde_json::from_str(
        &fs::read_to_string(&args.away).unwrap()
    ).unwrap();

    // Load the context from its file
    let context: GameContext = match &args.context {
        Some(x) => serde_json::from_str(&fs::read_to_string(x).unwrap()).unwrap(),
        None => GameContext::new()
    };

    // Instantiate the simulator
    let drive_sim = DriveSimulator::new();

    // Instantiate an RNG and simulate
    let mut rng = rand::thread_rng();
    let (drive, _new_context) = drive_sim.sim(
        &home_team,
        &away_team,
        context,
        &mut rng
    );

    // Serialize the drive result as a string based on the given output format
    //let output_format = OutputFormat::from_str(
    //    &args.output_format.clone().unwrap_or(String::from(""))
    //).unwrap();
    let drive_str: String = format!("{}", drive);

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
    }
}
