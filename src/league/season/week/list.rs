use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;

use crate::cli::league::season::week::FbsimLeagueSeasonWeekListArgs;

use serde_json;
use tabwriter::TabWriter;

pub fn list_season_weeks(args: FbsimLeagueSeasonWeekListArgs) -> Result<(), String> {
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

    // Display the season weeks in a table
    let mut tw = TabWriter::new(stdout());
    writeln!(&mut tw,"Week\tGames\tSimulated").map_err(|e| e.to_string())?;
    for (i, week) in season.weeks().iter().enumerate() {
        writeln!(
            &mut tw, "{}\t{}\t{}", i+1,
            week.matchups().len(),
            week.matchups().iter().filter(|m| m.context().game_over()).collect::<Vec<_>>().len()
        ).map_err(|e| e.to_string())?;
    }
    tw.flush().map_err(|e| e.to_string())?;
    Ok(())
}
