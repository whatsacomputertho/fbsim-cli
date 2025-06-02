use std::fs;

use fbsim_core::league::League;

use crate::cli::league::season::week::FbsimLeagueSeasonWeekGetArgs;

use serde_json;

pub fn get_season_week(args: FbsimLeagueSeasonWeekGetArgs) -> Result<(), String> {
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

    // TODO: Calculate a summary of the week
    // TODO: Display the summary in a nice looking way (not JSON)

    // Get the league season
    let season = match league.season(args.year) {
        Some(season) => season,
        None => return Err(format!("No season found with year: {}", args.year)),
    };

    // Get the league season week from the league season
    let week = match season.weeks().get(args.week) {
        Some(week) => week,
        None => return Err(format!("No week found in season {} with id: {}", args.year, args.week)),
    };

    // Serialize the season week as JSON
    let week_res = serde_json::to_string_pretty(&week);
    let week_str: String = match week_res {
        Ok(week_str) => week_str,
        Err(error) => return Err(format!("Error serializing season week: {}", error)),
    };

    // Print the week to stdout
    println!("{}", week_str);
    Ok(())
}
