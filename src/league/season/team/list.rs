use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;
use fbsim_core::league::season::matchup::LeagueSeasonMatchups;

use crate::cli::league::season::team::FbsimLeagueSeasonTeamListArgs;

use serde_json;
use tabwriter::TabWriter;

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

    // Get the league season
    let season = match league.season(args.year) {
        Some(season) => season,
        None => return Err(format!("No season found with year: {}", args.year)),
    };

    // Get the league season teams from the league season
    let teams = season.teams();
    let playoffs = season.playoffs();
    let playoffs_complete = playoffs.complete();
    let champion_id = playoffs.champion();

    // Display the results in a table
    let mut tw = TabWriter::new(stdout());
    if playoffs_complete {
        writeln!(&mut tw, "Team\tRecord\tPlayoffs\tChampion").map_err(|e| e.to_string())?;
    } else {
        writeln!(&mut tw, "Team\tRecord\tPlayoffs").map_err(|e| e.to_string())?;
    }

    for (id, team) in teams.iter() {
        let matchups: LeagueSeasonMatchups = season.team_matchups(*id)?;

        // Display playoff record based on whether team was in playoffs
        let playoff_record_str = match playoffs.record(*id) {
            Ok(playoff_record) => playoff_record.to_string(),
            Err(_) => String::from("-"),
        };

        if playoffs_complete {
            // Check if this team is the champion
            let champion_str = if champion_id == Some(*id) { "X" } else { "" };
            writeln!(&mut tw, "{}\t{}\t{}\t{}", team.name(), matchups.record(), playoff_record_str, champion_str).map_err(|e| e.to_string())?;
        } else {
            writeln!(&mut tw, "{}\t{}\t{}", team.name(), matchups.record(), playoff_record_str).map_err(|e| e.to_string())?;
        }
    }
    tw.flush().map_err(|e| e.to_string())?;
    Ok(())
}
