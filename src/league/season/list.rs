use std::fs;

use fbsim_core::league::League;
use fbsim_core::league::season::LeagueSeason;

use crate::cli::league::season::FbsimLeagueSeasonListArgs;

use serde_json;

pub fn list_seasons(args: FbsimLeagueSeasonListArgs) {
    // Load the league from its file
    let league: League = serde_json::from_str(
        &fs::read_to_string(&args.league).unwrap()
    ).unwrap();

    // TODO: Calculate a summary for each season
    // TODO: Display the results in a table

    // Get the current and past season from the league
    let current_season = league.current_season();
    let past_seasons: &Vec<LeagueSeason> = league.seasons();

    // Serialize the seasons as JSON
    let current_season_str: String = serde_json::to_string_pretty(&current_season).unwrap();
    let past_seasons_str: String = serde_json::to_string_pretty(&past_seasons).unwrap();

    // Print the seasons to the console
    println!("Current Season:");
    println!("{}", current_season_str);
    println!("");
    println!("Past Seasons:");
    println!("{}", past_seasons_str);
}
