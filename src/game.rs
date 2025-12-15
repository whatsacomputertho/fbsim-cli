pub mod play;
pub mod score;

use std::fs;
use std::str::FromStr;

use fbsim_core::game::play::PlaySimulator;
use fbsim_core::game::context::GameContext;
use fbsim_core::team::FootballTeam;

use crate::cli::game::FbsimGameSimArgs;

pub fn game_sim(args: FbsimGameSimArgs) {
    // Load the home and away teams from their files
    let home_team: FootballTeam = serde_json::from_str(
        &fs::read_to_string(&args.home).unwrap()
    ).unwrap();
    let away_team: FootballTeam = serde_json::from_str(
        &fs::read_to_string(&args.away).unwrap()
    ).unwrap();

    // Initialize a new context, simulator, and RNG
    let mut context: GameContext = GameContext::new();
    let play_sim = PlaySimulator::new();
    let mut rng = rand::thread_rng();

    // Simulate until the game is over
    let mut game_over: bool = false;
    while !game_over {
        game_over = *context.game_over();
        if !game_over {
            let (play, new_context) = play_sim.sim(&home_team, &away_team, context.clone(), &mut rng);
            println!("{}", play);
            context = new_context;
        } else {
            println!("{} Game over", context);
        }
    }
}
