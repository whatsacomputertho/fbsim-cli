use std::fs;

use fbsim_core::league::League;
use fbsim_core::league::season::LeagueSeason;

use crate::cli::league::season::FbsimLeagueSeasonListArgs;

use serde_json;

pub fn list_seasons(args: FbsimLeagueSeasonListArgs) -> Result<(), String> {
    // Load the league from its file
    let file_res = &fs::read_to_string(&args.league);
    let file = match file_res {
        Ok(file) => file,
        Err(error) => return Err(format!("Error loading league file: {}", error)),
    };
    let league_res = serde_json::from_str(file);
    let league: League = match league_res {
        Ok(league) => league,
        Err(error) => return Err(format!("Error loading league from file: {}", error)),
    };

    // TODO: Calculate a summary for each season
    // TODO: Display the results in a table

    // Get the current and past season from the league
    let current_season = league.current_season();
    let past_seasons: &Vec<LeagueSeason> = league.seasons();

    // Serialize the seasons as JSON
    let current_season_str_res = serde_json::to_string_pretty(&current_season);
    let current_season_str = match current_season_str_res {
        Ok(current_season_str) => current_season_str,
        Err(error) => return Err(format!("Error serializing current season: {}", error)),
    };
    let past_seasons_str_res = serde_json::to_string_pretty(&past_seasons);
    let past_seasons_str = match past_seasons_str_res {
        Ok(past_seasons_str) => past_seasons_str,
        Err(error) => return Err(format!("Error serializing past seasons: {}", error)),
    };

    // Print the seasons to the console
    println!("Current Season:");
    println!("{}", current_season_str);
    println!("");
    println!("Past Seasons:");
    println!("{}", past_seasons_str);
    Ok(())
}
