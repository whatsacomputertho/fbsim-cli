use std::fs;

use fbsim_core::league::League;

use crate::cli::league::season::playoffs::FbsimLeagueSeasonPlayoffsSimArgs;

use serde_json;

pub fn sim_playoffs(args: FbsimLeagueSeasonPlayoffsSimArgs) -> Result<(), String> {
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

    // Get the current season
    let season = match league.current_season_mut() {
        Some(s) => s,
        None => return Err(String::from("No current season found")),
    };

    // Simulate the playoffs
    let mut rng = rand::thread_rng();
    if let Err(e) = season.sim_playoffs(&mut rng) {
        return Err(format!("Failed to simulate playoffs: {}", e));
    }

    // Get the champion
    let champion_msg = if let Some(champion_id) = season.playoffs().champion() {
        let champion = season.team(champion_id).unwrap();
        format!("Champion: {}", champion.name())
    } else {
        String::from("Playoffs complete")
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

    println!("{}", champion_msg);
    Ok(())
}
