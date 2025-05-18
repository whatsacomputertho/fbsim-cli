use std::fs;

use fbsim_core::league::{League, LeagueTeam};

use crate::cli::league::team::FbsimLeagueTeamGetArgs;

use serde_json;

pub fn get_team(args: FbsimLeagueTeamGetArgs) {
    // Load the league from its file
    let league: League = serde_json::from_str(
        &fs::read_to_string(&args.league).unwrap()
    ).unwrap();

    // TODO: Calculate team performance in the league over time
    // TODO: Display the team's performance in a nice lookign way (not JSON)

    // Get a team from the league
    let team: &LeagueTeam = if let Some(x) = league.team(args.team) {
        x
    } else {
        println!("No team foud with ID: {}", args.team);
        return;
    };

    // Serialize the team as JSON
    let team_str: String = serde_json::to_string_pretty(&team).unwrap();

    // Print the team to the console
    println!("{}", team_str);
}
