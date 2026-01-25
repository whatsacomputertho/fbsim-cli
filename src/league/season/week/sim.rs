use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;

use crate::cli::league::season::week::FbsimLeagueSeasonWeekSimArgs;

use serde_json;
use tabwriter::TabWriter;

pub fn sim_season_week(args: FbsimLeagueSeasonWeekSimArgs) -> Result<(), String> {
    // Load the league from its file as mutable
    let file_res = &fs::read_to_string(&args.league);
    let file = match file_res {
        Ok(file) => file,
        Err(error) => return Err(format!("Error loading league file: {}", error)),
    };
    let league_res = serde_json::from_str(file);
    let mut league: League = match league_res {
        Ok(league) => league,
        Err(error) => return Err(format!("Error loading league from file: {}", error)),
    };

    // Simulate the league season week in the current league season
    let mut rng = rand::thread_rng();
    if let Err(e) = league.sim_week(args.week, &mut rng) {
        return Err(
            format!(
                "Failed to simulate week {}: {}",
                args.week, e
            )
        );
    }

    // Display results
    let season = match league.current_season() {
        Some(s) => s,
        None => return Err(String::from("No current season found")),
    };
    let week = match season.weeks().get(args.week) {
        Some(w) => w,
        None => return Err(format!("No week found with index: {}", args.week)),
    };

    println!("Week {} Results", args.week);
    let mut tw = TabWriter::new(stdout());
    writeln!(&mut tw, "Matchup\tAway Team\tAway Score\tHome Team\tHome Score").map_err(|e| e.to_string())?;
    for (i, matchup) in week.matchups().iter().enumerate() {
        let away_team = season.team(*matchup.away_team()).unwrap().name();
        let home_team = season.team(*matchup.home_team()).unwrap().name();
        let context = matchup.context();
        writeln!(
            &mut tw, "{}\t{}\t{}\t{}\t{}",
            i,
            away_team, context.away_score(),
            home_team, context.home_score()
        ).map_err(|e| e.to_string())?;
    }
    tw.flush().map_err(|e| e.to_string())?;

    // Serialize the league as JSON
    let league_res = serde_json::to_string_pretty(&league);
    let league_str: String = match league_res {
        Ok(league_str) => league_str,
        Err(error) => return Err(format!("Error serializing league: {}", error)),
    };

    // Write the league back to its file
    let write_res = fs::write(&args.league, league_str);
    if let Err(e) = write_res {
        return Err(format!("Error writing league file: {}", e));
    }
    Ok(())
}
