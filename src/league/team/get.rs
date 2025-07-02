use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;
use fbsim_core::league::matchup::LeagueMatchups;

use crate::cli::league::team::FbsimLeagueTeamGetArgs;

use serde_json;
use tabwriter::TabWriter;

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

    // Check whether the team exists
    let _team = match league.team(args.team) {
        Some(t) => t,
        None => return Err(format!("No team found with ID: {}", args.team))
    };

    // Get the team's matchups, if none then the team has never participated
    let matchups: LeagueMatchups = league.team_matchups(args.team);
    if matchups.matchups().len() < 1 {
        return Err(format!("Team with ID {} has not participated in a season", args.team));
    }

    // If matchups exist, then summarize
    let mut tw = TabWriter::new(stdout());
    write!(&mut tw, "Year\tTeam\tRecord\n").map_err(|e| e.to_string())?;

    // Calculate the team's record for each previous season
    // Get the team's name for each previous season
    for (year, season) in matchups.matchups().iter() {
        let team = match league.season(*year).unwrap().team(args.team) {
            Some(t) => t,
            None => continue
        };
        write!(&mut tw, "{}\t{}\t{}\n", year, team.name(), season.record()).map_err(|e| e.to_string())?;
    }
    write!(&mut tw, "Total\t\t{}\n", matchups.record()).map_err(|e| e.to_string())?;
    tw.flush().map_err(|e| e.to_string())?;
    Ok(())
}
