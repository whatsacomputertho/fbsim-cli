use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;

use crate::cli::league::season::team::FbsimLeagueSeasonTeamGetArgs;

use serde_json;
use tabwriter::TabWriter;

pub fn get_season_team(args: FbsimLeagueSeasonTeamGetArgs) -> Result<(), String> {
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

    // Get the league season team from the league season
    let team = match season.team(args.id) {
        Some(team) => team,
        None => return Err(format!("No team found in season {} with id: {}", args.year, args.id)),
    };

    // Get the league season team's matchups from the league season
    let matchups = season.team_matchups(args.id)?;

    // Display the results in a table
    let mut tw = TabWriter::new(stdout());
    writeln!(&mut tw, "Team:\t{}", team.name()).map_err(|e| e.to_string())?;
    writeln!(&mut tw, "Record:\t{}\n", matchups.record()).map_err(|e| e.to_string())?;

    // Display each matchup
    writeln!(
        &mut tw,
        "Week\tHome Team\tHome Score\tAway Team\tAway Score"
    ).map_err(|e| e.to_string())?;
    for (i, matchup) in matchups.matchups().iter().enumerate() {
        match matchup {
            Some(m) => {
                let context = m.context();
                let away_team = season.team(*m.away_team()).unwrap().name();
                let home_team = season.team(*m.home_team()).unwrap().name();
                writeln!(
                    &mut tw, "{}\t{}\t{}\t{}\t{}", i+1,
                    home_team, context.home_score(),
                    away_team, context.away_score()
                ).map_err(|e| e.to_string())?;
            },
            None => {
                writeln!(
                    &mut tw, "{}\tBYE", i+1
                ).map_err(|e| e.to_string())?;
            },
        }
    }
    tw.flush().map_err(|e| e.to_string())?;
    Ok(())
}
