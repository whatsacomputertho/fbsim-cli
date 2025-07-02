use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;
use fbsim_core::league::season::LeagueSeason;

use crate::cli::league::season::FbsimLeagueSeasonListArgs;

use serde_json;
use tabwriter::TabWriter;

pub fn list_seasons(args: FbsimLeagueSeasonListArgs) -> Result<(), String> {
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

    // Get the current and past season from the league
    let current_season = league.current_season();
    let past_seasons: &Vec<LeagueSeason> = league.seasons();

    // Display the season list in a table
    let mut tw = TabWriter::new(stdout());
    write!(
        &mut tw,
        "Season\tTeams\tWeeks\n"
    ).map_err(|e| e.to_string())?;
    match current_season {
        Some(s) => {
            write!(
                &mut tw,
                "{} (Current)\t{}\t{}\n",
                s.year(), s.teams().len(), s.weeks().len()
            ).map_err(|e| e.to_string())?;
        },
        None => ()
    }
    for season in past_seasons.iter() {
        write!(
            &mut tw,
            "{}\t{}\t{}\n",
            season.year(),
            season.teams().len(),
            season.weeks().len()
        ).map_err(|e| e.to_string())?;
    }
    tw.flush().map_err(|e| e.to_string())?;
    Ok(())
}
