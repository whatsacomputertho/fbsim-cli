use std::fs;

use fbsim_core::league::League;

use crate::cli::league::season::team::FbsimLeagueSeasonTeamListArgs;

use serde_json;

pub fn list_season_teams(args: FbsimLeagueSeasonTeamListArgs) -> Result<(), String> {
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

    // TODO: Calculate a summary of each team's performance over the season
    // TODO: Display the summary in a nice looking way (not JSON)

    // Get the league season
    let season = match league.season(args.year) {
        Some(season) => season,
        None => return Err(format!("No season found with year: {}", args.year)),
    };

    // Get the league season teams from the league season
    let teams = season.teams();

    // Serialize the season team as JSON
    let teams_res = serde_json::to_string_pretty(&teams);
    let teams_str: String = match teams_res {
        Ok(teams_str) => teams_str,
        Err(error) => return Err(format!("Error serializing season team: {}", error)),
    };

    // Print the team to stdout
    println!("{}", teams_str);
    Ok(())
}
