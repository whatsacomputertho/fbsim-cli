use std::fs;

use fbsim_core::league::League;
use fbsim_core::league::season::conference::LeagueDivision;

use crate::cli::league::season::conference::division::FbsimLeagueSeasonConferenceDivisionAddArgs;

use serde_json;

pub fn add_division(args: FbsimLeagueSeasonConferenceDivisionAddArgs) -> Result<(), String> {
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

    // Get the current season and conference
    let season = match league.current_season_mut() {
        Some(s) => s,
        None => return Err(String::from("No current season found")),
    };
    let conference = match season.conference_mut(args.conference) {
        Some(c) => c,
        None => return Err(format!("No conference found with index: {}", args.conference)),
    };

    // Get the division ID and conference name before adding
    let div_id = conference.divisions().len();
    let conf_name = conference.name_mut().clone();

    // Add the division
    let division = LeagueDivision::with_name(&args.name);
    conference.add_division(division)?;

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
    println!("Division {} added to conference {} with ID {}", args.name, conf_name, div_id);
    Ok(())
}
