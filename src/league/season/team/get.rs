use std::fs;

use fbsim_core::league::League;

use crate::cli::league::season::team::FbsimLeagueSeasonTeamGetArgs;

use serde_json;

pub fn get_season_team(args: FbsimLeagueSeasonTeamGetArgs) -> Result<(), String> {
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

    // TODO: Calculate a summary of the team's performance over the season
    // TODO: Display the summary in a nice looking way (not JSON)

    // Get the league season
    let season = match league.season(args.year) {
        Some(season) => season,
        None => return Err(format!("No season found with year: {}", args.year)),
    };

    // Get the league season team from the league season
    let team = match season.team(args.id) {
        Some(team) => team,
        None => return Err(format!("No team found in season {} with id: {}", args.year, args.id)),
    };

    // Serialize the season team as JSON
    let team_res = serde_json::to_string_pretty(&team);
    let team_str: String = match team_res {
        Ok(team_str) => team_str,
        Err(error) => return Err(format!("Error serializing season team: {}", error)),
    };

    // Print the team to stdout
    println!("{}", team_str);
    Ok(())
}
