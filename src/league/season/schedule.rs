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

    // Validate conference-based options
    let has_conference_options = args.division_games.is_some()
        || args.conference_games.is_some()
        || args.cross_conference_games.is_some();

    if has_conference_options {
        let season = match league.current_season() {
            Some(s) => s,
            None => return Err(String::from("No current season found")),
        };
        if season.conferences().is_empty() {
            return Err(String::from(
                "Conference-based schedule options (--division-games, --conference-games, --cross-conference-games) \
                require conferences to be defined. Use 'league season conference add' first."
            ));
        }
    }

    // Initialize schedule gen options
    let options = LeagueSeasonScheduleOptions{
        weeks: args.weeks,
        shift: args.shift,
        permute: args.permute,
        division_games: args.division_games,
        conference_games: args.conference_games,
        cross_conference_games: args.cross_conference_games,
    };

    // Attempt to generate a schedule for the league
    let mut rng = match args.seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => match StdRng::from_rng(rand::thread_rng()) {
            Ok(rng) => rng,
            Err(error) => return Err(format!("Failed to instantiate rng: {}", error)),
        },
    };
    if let Err(e) = league.generate_schedule(options, &mut rng) {
        return Err(format!("Error generating league schedule: {}", e));
    }

    // Get schedule info for confirmation message
    let num_weeks = match league.current_season() {
        Some(s) => s.weeks().len(),
        None => 0,
    };

    // Serialize the league as JSON
    let league_str: String = match serde_json::to_string_pretty(&league) {
        Ok(league_str) => league_str,
        Err(error) => return Err(format!("Error serializing league: {}", error)),
    };

    // Write the league back to its file
    let write_res = fs::write(&args.league, league_str);
    if let Err(e) = write_res {
        return Err(format!("Error writing league file: {}", e));
    };

    println!("Schedule generated with {} weeks", num_weeks);
    Ok(())
}
