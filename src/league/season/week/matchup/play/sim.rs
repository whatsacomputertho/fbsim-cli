use std::fs;

use fbsim_core::league::League;

use crate::cli::league::season::week::matchup::play::FbsimLeagueSeasonWeekMatchupPlaySimArgs;

use serde_json;

pub fn sim_play(args: FbsimLeagueSeasonWeekMatchupPlaySimArgs) -> Result<(), String> {
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

    // Simulate the matchup
    let mut rng = rand::thread_rng();
    let _ = match league.sim_play(args.week, args.matchup, &mut rng) {
        Ok(opt) => opt,
        Err(error) => return Err(format!("Error simulating next play for matchup: {}", error)),
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
