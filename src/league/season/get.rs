use std::fs;

use fbsim_core::league::League;
use fbsim_core::league::season::LeagueSeason;

use crate::cli::league::season::FbsimLeagueSeasonGetArgs;

use serde_json;

pub fn get_season(args: FbsimLeagueSeasonGetArgs) -> Result<(), String> {
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

    // TODO: Calculate a summary of the season
    // TODO: Display the season summary in a nice looking way (not JSON)

    // Get a season from the league
    let season: &LeagueSeason = match league.season(args.year) {
        Some(season) => season,
        None => return Err(format!("No season found with year: {}", args.year)),
    };

    // Serialize the season as JSON
    let season_str_res = serde_json::to_string_pretty(&season);
    let season_str = match season_str_res {
        Ok(season_str) => season_str,
        Err(error) => return Err(format!("Error serializing season: {}", error))
    };

    // Print the season to the console
    println!("{}", season_str);
    Ok(())
}
