use std::fs;
use std::collections::BTreeMap;

use fbsim_core::league::League;
use fbsim_core::league::team::LeagueTeam;

use crate::cli::league::team::FbsimLeagueTeamListArgs;

use serde_json;

pub fn list_teams(args: FbsimLeagueTeamListArgs) {
    // Load the league from its file
    let league: League = serde_json::from_str(
        &fs::read_to_string(&args.league).unwrap()
    ).unwrap();

    // TODO: Calculate each team's performance in the league over time
    // TODO: Display the results in a table

    // Get the collection of teams from the league
    let teams: &BTreeMap<usize, LeagueTeam> = league.teams();

    // Serialize the teams as JSON
    let teams_str: String = serde_json::to_string_pretty(&teams).unwrap();

    // Print the teams to the console
    println!("{}", teams_str);
}
