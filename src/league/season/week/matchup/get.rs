use std::fs;

use fbsim_core::league::League;

use crate::cli::league::season::week::matchup::FbsimLeagueSeasonWeekMatchupGetArgs;

use serde_json;

pub fn get_matchup(args: FbsimLeagueSeasonWeekMatchupGetArgs) -> Result<(), String> {
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

    // TODO: Calculate a summary of the matchup
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

    // Get the league season matchup from the league week
    let matchup = match week.matchups().get(args.matchup) {
        Some(matchup) => matchup,
        None => return Err(
            format!(
                "No matchup found in season {} week {} with id: {}",
                args.year, args.week, args.matchup
            )
        ),
    };

    // Serialize the matchup as JSON
    let matchup_res = serde_json::to_string_pretty(&matchup);
    let matchup_str: String = match matchup_res {
        Ok(matchup_str) => matchup_str,
        Err(error) => return Err(format!("Error serializing matchup: {}", error)),
    };

    // Print the matchup to stdout
    println!("{}", matchup_str);
    Ok(())
}
