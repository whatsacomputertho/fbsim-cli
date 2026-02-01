use std::fs;

use fbsim_core::league::League;

use crate::cli::league::season::playoffs::FbsimLeagueSeasonPlayoffsGetArgs;
use crate::league::season::playoffs::display;

use serde_json;

pub fn get_playoffs(args: FbsimLeagueSeasonPlayoffsGetArgs) -> Result<(), String> {
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

    // Get the season
    let season = match league.season(args.year) {
        Some(season) => season,
        None => return Err(format!("No season found with year: {}", args.year)),
    };

    // Display general playoff info
    let playoffs = season.playoffs();
    println!("Playoffs for {} season ({} teams)", args.year, playoffs.num_teams());
    println!();
    display::display_playoffs(season)
}
