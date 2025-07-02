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
    write!(&mut tw, "Team:\t{}\n", team.name()).map_err(|e| e.to_string())?;
    write!(&mut tw, "Record:\t{}\n\n", matchups.record()).map_err(|e| e.to_string())?;

    // Display each matchup
    write!(
        &mut tw,
        "Week\tHome Team\tHome Score\tAway Team\tAway Score\n"
    ).map_err(|e| e.to_string())?;
    for (i, matchup) in matchups.matchups().iter().enumerate() {
        match matchup {
            Some(m) => {
                let away_team = season.team(*m.away_team()).unwrap().name();
                let home_team = season.team(*m.home_team()).unwrap().name();
                write!(
                    &mut tw, "{}\t{}\t{}\t{}\t{}\n", i+1,
                    home_team, m.home_score(),
                    away_team, m.away_score()
                ).map_err(|e| e.to_string())?;
            },
            None => {
                write!(
                    &mut tw, "{}\t{}", i+1, "BYE"
                ).map_err(|e| e.to_string())?;
            },
        }
    }
    tw.flush().map_err(|e| e.to_string())?;
    Ok(())
}
