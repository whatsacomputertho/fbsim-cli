use std::fs;

use fbsim_core::league::League;
use fbsim_core::league::season::LeagueSeasonScheduleOptions;

use crate::cli::league::season::schedule::FbsimLeagueSeasonScheduleGenArgs;

use rand::{SeedableRng};
use rand::rngs::StdRng;
use serde_json;

pub fn generate_schedule(args: FbsimLeagueSeasonScheduleGenArgs) -> Result<(), String> {
    // Load the league from its file
    let file_res = &fs::read_to_string(&args.league);
    let file = match file_res {
        Ok(file) => file,
        Err(error) => return Err(format!("Error loading league file: {}", error)),
    };
    let mut league: League = match serde_json::from_str(file) {
        Ok(league) => league,
        Err(error) => return Err(format!("Error loading league from file: {}", error)),
    };

    // Initialize schedule gen options
    let options = LeagueSeasonScheduleOptions{
        weeks: args.weeks,
        shift: args.shift,
        permute: args.permute,
    };

    // Attempt to generate a schedule for the league
    let mut rng = match args.seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => match StdRng::from_rng(rand::thread_rng()) {
            Ok(rng) => rng,
            Err(error) => return Err(format!("Failed to instantiate rng: {}", error)),
        },
    };
    let _schedule_gen = match league.generate_schedule(options, &mut rng) {
        Ok(()) => (),
        Err(error) => return Err(format!("Error generating league schedule: {}", error)),
    };

    // Serialize the league as JSON
    let league_str: String = match serde_json::to_string_pretty(&league) {
        Ok(league_str) => league_str,
        Err(error) => return Err(format!("Error serializing league: {}", error)),
    };

    // Write the league back to its file
    let write_res = fs::write(&args.league, league_str);
    let _file_write = match write_res {
        Ok(()) => (),
        Err(error) => return Err(format!("Error writing league file: {}", error)),
    };
    Ok(())
}
