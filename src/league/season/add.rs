use std::fs;

use fbsim_core::league::League;

use crate::cli::league::season::FbsimLeagueSeasonAddArgs;

use serde_json;

pub fn add_season(args: FbsimLeagueSeasonAddArgs) -> Result<(), String> {
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

    // Add a new season to the league
    let season_res = league.add_season();
    if let Err(e) = season_res {
        return Err(format!("Error adding season: {}", e));
    }

    // Get the new season year for confirmation message
    let year = match league.current_season() {
        Some(s) => *s.year(),
        None => 0,
    };

    // Serialize the league as JSON
    let league_str_res = serde_json::to_string_pretty(&league);
    let league_str: String = match league_str_res {
        Ok(league_str) => league_str,
        Err(error) => return Err(format!("Error serializing league: {}", error)),
    };

    // Write the league back to its file
    let write_res = fs::write(&args.league, league_str);
    if let Err(e) = write_res {
        return Err(format!("Error writing league file: {}", e));
    }

    println!("Season {} added to league", year);
    Ok(())
}
