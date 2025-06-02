use std::fs;

use fbsim_core::league::League;

use crate::cli::league::season::week::FbsimLeagueSeasonWeekListArgs;

use serde_json;

pub fn list_season_weeks(args: FbsimLeagueSeasonWeekListArgs) -> Result<(), String> {
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

    // TODO: Calculate a summary of each week
    // TODO: Display the summary in a nice looking way (not JSON)

    // Get the league season
    let season = match league.season(args.year) {
        Some(season) => season,
        None => return Err(format!("No season found with year: {}", args.year)),
    };

    // Get the league season weeks from the league season
    let weeks = season.weeks();

    // Serialize the season weeks as JSON
    let weeks_res = serde_json::to_string_pretty(&weeks);
    let weeks_str: String = match weeks_res {
        Ok(weeks_str) => weeks_str,
        Err(error) => return Err(format!("Error serializing season weeks: {}", error)),
    };

    // Print the weeks to stdout
    println!("{}", weeks_str);
    Ok(())
}
