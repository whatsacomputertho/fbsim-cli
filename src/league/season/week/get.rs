use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;

use crate::cli::league::season::week::FbsimLeagueSeasonWeekGetArgs;

use serde_json;
use tabwriter::TabWriter;

pub fn get_season_week(args: FbsimLeagueSeasonWeekGetArgs) -> Result<(), String> {
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

    // Get the league season week from the league season
    let week = match season.weeks().get(args.week) {
        Some(week) => week,
        None => return Err(format!("No week found in season {} with id: {}", args.year, args.week)),
    };

    // Display each matchup in the week in a table
    let mut tw = TabWriter::new(stdout());
    writeln!(&mut tw,"Matchup\tAway Team\tAway Score\tHome Team\tHome Score").map_err(|e| e.to_string())?;
    for (i, matchup) in week.matchups().iter().enumerate() {
        // Get the team names
        let away_team = season.team(*matchup.away_team()).unwrap().name();
        let home_team = season.team(*matchup.home_team()).unwrap().name();

        // Get the game context
        let context = matchup.context();

        // Display the matchup
        writeln!(
            &mut tw,"{}\t{}\t{}\t{}\t{}", i,
            away_team, context.away_score(),
            home_team, context.home_score()
        ).map_err(|e| e.to_string())?;
    }
    tw.flush().map_err(|e| e.to_string())?;
    Ok(())
}
