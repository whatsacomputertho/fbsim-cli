use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;

use crate::cli::league::season::conference::division::FbsimLeagueSeasonConferenceDivisionListArgs;

use serde_json;
use tabwriter::TabWriter;

pub fn list_divisions(args: FbsimLeagueSeasonConferenceDivisionListArgs) -> Result<(), String> {
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

    // Get the season, conference, and divisions
    let season = match league.season(args.year) {
        Some(season) => season,
        None => return Err(format!("No season found with year: {}", args.year)),
    };
    let conferences = season.conferences();
    let conference = match conferences.get(args.conference) {
        Some(c) => c,
        None => return Err(format!("No conference found with ID: {}", args.conference)),
    };
    let divisions = conference.divisions();
    if divisions.is_empty() {
        println!("No divisions found in conference {}", conference.name());
        return Ok(());
    }

    // Display divisions in a table
    println!("=== {} Divisions ===", conference.name());
    let mut tw = TabWriter::new(stdout());
    writeln!(&mut tw, "ID\tName\tTeams").map_err(|e| e.to_string())?;
    for (div_id, division) in divisions.iter().enumerate() {
        writeln!(
            &mut tw, "{}\t{}\t{}",
            div_id,
            division.name(),
            division.teams().len()
        ).map_err(|e| e.to_string())?;
    }
    tw.flush().map_err(|e| e.to_string())?;
    Ok(())
}
