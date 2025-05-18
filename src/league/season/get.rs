use std::fs;

use fbsim_core::league::{League, LeagueSeason};

use crate::cli::league::season::FbsimLeagueSeasonGetArgs;

use serde_json;

pub fn get_season(args: FbsimLeagueSeasonGetArgs) {
    // Load the league from its file
    let league: League = serde_json::from_str(
        &fs::read_to_string(&args.league).unwrap()
    ).unwrap();

    // TODO: Calculate a summary of the season
    // TODO: Display the season summary in a nice looking way (not JSON)

    // Get a season from the league
    let season: &LeagueSeason = if let Some(x) = league.season(args.year) {
        x
    } else {
        println!("No season foud with year: {}", args.year);
        return;
    };

    // Serialize the season as JSON
    let season_str: String = serde_json::to_string_pretty(&season).unwrap();

    // Print the season to the console
    println!("{}", season_str);
}
