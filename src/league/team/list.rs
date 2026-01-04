use std::fs;
use std::io::{Write, stdout};
use std::collections::BTreeMap;

use fbsim_core::league::League;
use fbsim_core::league::team::LeagueTeam;
use fbsim_core::league::matchup::LeagueMatchups;

use crate::cli::league::team::FbsimLeagueTeamListArgs;

use serde_json;
use tabwriter::TabWriter;

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

    // Display the results in a table
    let mut tw = TabWriter::new(stdout());
    writeln!(&mut tw, "ID\tTeam\tSeasons\tRecord").map_err(|e| e.to_string())?;

    // Get the collection of teams from the league
    let teams: &BTreeMap<usize, LeagueTeam> = league.teams();
    for (id, _) in teams.iter() {
        let matchups: LeagueMatchups = league.team_matchups(*id);

        // Get the most recent team name
        let team = if !matchups.matchups().is_empty() {
            if let Some((year, _)) = matchups.matchups().iter().next_back() {
                league.season(*year).unwrap().team(*id).unwrap().name().clone()
            } else {
                "(No Name)".to_string()
            }
        } else {
            "(No Name)".to_string()
        };

        writeln!(&mut tw, "{}\t{}\t{}\t{}", id, team, matchups.matchups().len(), matchups.record()).map_err(|e| e.to_string())?;
    }
    tw.flush().map_err(|e| e.to_string())?;
    Ok(())
}
