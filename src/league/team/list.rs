use std::fs;
use std::collections::BTreeMap;

use fbsim_core::league::League;
use fbsim_core::league::team::LeagueTeam;

use crate::cli::league::team::FbsimLeagueTeamListArgs;

use serde_json;

pub fn list_teams(args: FbsimLeagueTeamListArgs) -> Result<(), String> {
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

    // TODO: Calculate each team's performance in the league over time
    // TODO: Display the results in a table

    // Get the collection of teams from the league
    let teams: &BTreeMap<usize, LeagueTeam> = league.teams();

    // Serialize the teams as JSON
    let teams_str_res = serde_json::to_string_pretty(&teams);
    let teams_str = match teams_str_res {
        Ok(teams_str) => teams_str,
        Err(error) => return Err(format!("Error serializing teams: {}", error)),
    };

    // Print the teams to the console
    println!("{}", teams_str);
    Ok(())
}
