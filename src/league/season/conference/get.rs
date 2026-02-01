use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;

use crate::cli::league::season::conference::FbsimLeagueSeasonConferenceGetArgs;

use serde_json;
use tabwriter::TabWriter;

pub fn get_conference(args: FbsimLeagueSeasonConferenceGetArgs) -> Result<(), String> {
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

    // Get the season and conference
    let season = match league.season(args.year) {
        Some(season) => season,
        None => return Err(format!("No season found with year: {}", args.year)),
    };
    let conferences = season.conferences();
    let conference = match conferences.get(args.conference) {
        Some(c) => c,
        None => return Err(format!("No conference found with ID: {}", args.conference)),
    };

    // Display conference divisions in a table
    println!("=== {} ===", conference.name());
    let mut tw = TabWriter::new(stdout());
    if conference.divisions().is_empty() {
        writeln!(&mut tw, "No divisions").map_err(|e| e.to_string())?;
    } else {
        writeln!(&mut tw, "Divisions:").map_err(|e| e.to_string())?;
        writeln!(&mut tw, "ID\tName\tTeams").map_err(|e| e.to_string())?;
        for (div_id, division) in conference.divisions().iter().enumerate() {
            writeln!(
                &mut tw, "{}\t{}\t{}",
                div_id,
                division.name(),
                division.teams().len()
            ).map_err(|e| e.to_string())?;
        }
    }

    // Display teams in conference
    let team_ids = conference.all_teams();
    if !team_ids.is_empty() {
        writeln!(&mut tw).map_err(|e| e.to_string())?;
        writeln!(&mut tw, "Teams:").map_err(|e| e.to_string())?;
        writeln!(&mut tw, "ID\tName\tDivision\tRecord").map_err(|e| e.to_string())?;
        for division in conference.divisions() {
            for team_id in division.teams() {
                if let Some(team) = season.team(*team_id) {
                    let record = season.team_matchups(*team_id)
                        .map(|m| m.record().to_string())
                        .unwrap_or_else(|_| String::from("-"));
                    writeln!(
                        &mut tw, "{}\t{}\t{}\t{}",
                        team_id,
                        team.name(),
                        division.name(),
                        record
                    ).map_err(|e| e.to_string())?;
                }
            }
        }
    }
    tw.flush().map_err(|e| e.to_string())?;
    Ok(())
}
