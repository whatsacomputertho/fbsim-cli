use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;

use crate::cli::league::season::conference::division::FbsimLeagueSeasonConferenceDivisionGetArgs;

use serde_json;
use tabwriter::TabWriter;

pub fn get_division(args: FbsimLeagueSeasonConferenceDivisionGetArgs) -> Result<(), String> {
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

    // Get the season, conference, and division
    let season = match league.season(args.year) {
        Some(season) => season,
        None => return Err(format!("No season found with year: {}", args.year)),
    };
    let conferences = season.conferences();
    let conference = match conferences.get(args.conference) {
        Some(c) => c,
        None => return Err(format!("No conference found with ID: {}", args.conference)),
    };
    let division = match conference.division(args.division) {
        Some(d) => d,
        None => return Err(format!("No division found with ID: {}", args.division)),
    };

    // Display division info
    let mut tw = TabWriter::new(stdout());
    writeln!(&mut tw, "Division:\t{}", division.name()).map_err(|e| e.to_string())?;
    writeln!(&mut tw, "Conference:\t{}", conference.name()).map_err(|e| e.to_string())?;
    writeln!(&mut tw).map_err(|e| e.to_string())?;

    // Display teams in division
    let team_ids = division.teams();
    if team_ids.is_empty() {
        writeln!(&mut tw, "No teams assigned to this division").map_err(|e| e.to_string())?;
    } else {
        writeln!(&mut tw, "Teams:").map_err(|e| e.to_string())?;
        writeln!(&mut tw, "ID\tName\tRecord").map_err(|e| e.to_string())?;
        for team_id in team_ids {
            if let Some(team) = season.team(*team_id) {
                let record = season.team_matchups(*team_id)
                    .map(|m| m.record().to_string())
                    .unwrap_or_else(|_| String::from("-"));
                writeln!(
                    &mut tw, "{}\t{}\t{}",
                    team_id,
                    team.name(),
                    record
                ).map_err(|e| e.to_string())?;
            }
        }
    }
    tw.flush().map_err(|e| e.to_string())?;
    Ok(())
}
