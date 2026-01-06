use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;
use fbsim_core::league::season::LeagueSeason;

use crate::cli::league::season::FbsimLeagueSeasonGetArgs;

use serde_json;
use tabwriter::TabWriter;

pub fn get_season(args: FbsimLeagueSeasonGetArgs) -> Result<(), String> {
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

    // Get a season from the league
    let season: &LeagueSeason = match league.season(args.year) {
        Some(season) => season,
        None => return Err(format!("No season found with year: {}", args.year)),
    };

    // Display the season teams in a table
    let mut tw = TabWriter::new(stdout());
    writeln!(&mut tw,"Team\tRecord").map_err(|e| e.to_string())?;
    for (id, team) in season.teams().iter() {
        let matchups = season.team_matchups(*id)?;
        writeln!(
            &mut tw, "{}\t{}",
            team.name(), matchups.record()
        ).map_err(|e| e.to_string())?;
    }

    // Display the season weeks in a table
    writeln!(&mut tw,"\nWeek\tGames\tSimulated").map_err(|e| e.to_string())?;
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
