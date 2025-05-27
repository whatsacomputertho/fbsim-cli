use std::fs;

use fbsim_core::league::League;
use fbsim_core::league::team::LeagueTeam;

use crate::cli::league::team::FbsimLeagueTeamGetArgs;

use serde_json;

pub fn get_team(args: FbsimLeagueTeamGetArgs) -> Result<(), String> {
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

    // TODO: Calculate team performance in the league over time
    // TODO: Display the team's performance in a nice looking way (not JSON)

    // Get a team from the league
    let team: &LeagueTeam = match league.team(args.team) {
        Some(team) => team,
        None => return Err(format!("No team found with ID: {}", args.team)),
    };

    // Serialize the team as JSON
    let team_str_res = serde_json::to_string_pretty(&team);
    let team_str = match team_str_res {
        Ok(team_str) => team_str,
        Err(error) => return Err(format!("Error serializing team: {}", error)),
    };

    // Print the team to the console
    println!("{}", team_str);
    Ok(())
}
