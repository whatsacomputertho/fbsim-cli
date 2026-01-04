use std::fs;

use fbsim_core::league::League;

use crate::cli::league::season::week::matchup::FbsimLeagueSeasonWeekMatchupSimArgs;

use serde_json;

pub fn sim_matchup(args: FbsimLeagueSeasonWeekMatchupSimArgs) -> Result<(), String> {
    // Load the league from its file as mutable
    let file_res = &fs::read_to_string(&args.league);
    let file = match file_res {
        Ok(file) => file,
        Err(error) => return Err(format!("Error loading league file: {}", error)),
    };
    let league_res = serde_json::from_str(file);
    let mut league: League = match league_res {
        Ok(league) => league,
        Err(error) => return Err(format!("Error loading league from file: {}", error)),
    };

    // TODO: Calculate a summary of the matchup
    // TODO: Display the summary in a nice looking way (not JSON)

    // Simulate the matchup
    let mut rng = rand::thread_rng();
    match league.sim_matchup(args.week, args.matchup, &mut rng) {
        Ok(()) => (),
        Err(error) => return Err(format!("Error simulating matchup: {}", error)),
    };

    // Serialize the league as JSON
    let league_res = serde_json::to_string_pretty(&league);
    let league_str: String = match league_res {
        Ok(league_str) => league_str,
        Err(error) => return Err(format!("Error serializing league: {}", error)),
    };

    // Write the league back to its file
    let write_res = fs::write(&args.league, league_str);
    if let Err(e) = write_res {
        return Err(format!("Error writing league file: {}", e));
    }
    Ok(())
}
