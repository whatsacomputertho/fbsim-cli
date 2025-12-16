use std::fs;

use fbsim_core::game::play::PlaySimulator;
use fbsim_core::game::context::GameContext;
use fbsim_core::team::FootballTeam;

use crate::cli::game::play::FbsimGamePlaySimArgs;

use serde_json;

pub fn play_sim(args: FbsimGamePlaySimArgs) {
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
    let play_sim = PlaySimulator::new();

    // Instantiate an RNG and simulate
    let mut rng = rand::thread_rng();
    let (play, _new_context) = play_sim.sim(
        &home_team,
        &away_team,
        context,
        &mut rng
    );

    // Serialize the play result as a string based on the given output format
    //let output_format = OutputFormat::from_str(
    //    &args.output_format.clone().unwrap_or(String::from(""))
    //).unwrap();
    let play_str: String = format!("{}", play);

    // Write the play result either to stdout or to a file
    match &args.output_file {
        Some(x) => {
            // Write the output to the output file
            _ = fs::write(x, play_str);
        },
        None => {
            // Print the output to stdout
            println!("{}", play_str);
        }
    }
}
