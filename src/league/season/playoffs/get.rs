use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;

use crate::cli::league::season::playoffs::FbsimLeagueSeasonPlayoffsGetArgs;

use serde_json;
use tabwriter::TabWriter;

pub fn get_playoffs(args: FbsimLeagueSeasonPlayoffsGetArgs) -> Result<(), String> {
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

    // Get the season
    let season = match league.season(args.year) {
        Some(season) => season,
        None => return Err(format!("No season found with year: {}", args.year)),
    };

    // Get the playoffs
    let playoffs = season.playoffs();

    // Display playoff status
    println!("Playoffs for {} season ({} teams)", args.year, playoffs.num_teams());
    println!();

    // Display conference brackets
    for (conf_index, rounds) in playoffs.conference_brackets() {
        let conf_name = season.conferences().get(*conf_index)
            .map(|c| c.name().to_string())
            .unwrap_or_else(|| format!("Conference {}", conf_index));
        println!("=== {} ===", conf_name);

        for (round_index, round) in rounds.iter().enumerate() {
            println!("Round {}", round_index);
            let mut tw = TabWriter::new(stdout());
            writeln!(&mut tw, "Matchup\tAway Team\tAway Score\tHome Team\tHome Score\tStatus").map_err(|e| e.to_string())?;
            for (matchup_index, matchup) in round.matchups().iter().enumerate() {
                let away_team = season.team(*matchup.away_team()).unwrap().name();
                let home_team = season.team(*matchup.home_team()).unwrap().name();
                let context = matchup.context();
                let status = if context.game_over() {
                    "Final"
                } else if context.started() {
                    "In Progress"
                } else {
                    "Pending"
                };
                writeln!(
                    &mut tw, "{}\t{}\t{}\t{}\t{}\t{}",
                    matchup_index,
                    away_team, context.away_score(),
                    home_team, context.home_score(),
                    status
                ).map_err(|e| e.to_string())?;
            }
            tw.flush().map_err(|e| e.to_string())?;
            println!();
        }
    }

    // Display winners bracket
    let winners = playoffs.winners_bracket();
    if !winners.is_empty() {
        println!("=== Championship Bracket ===");
        for (round_index, round) in winners.iter().enumerate() {
            println!("Round {}", round_index);
            let mut tw = TabWriter::new(stdout());
            writeln!(&mut tw, "Matchup\tAway Team\tAway Score\tHome Team\tHome Score\tStatus").map_err(|e| e.to_string())?;
            for (matchup_index, matchup) in round.matchups().iter().enumerate() {
                let away_team = season.team(*matchup.away_team()).unwrap().name();
                let home_team = season.team(*matchup.home_team()).unwrap().name();
                let context = matchup.context();
                let status = if context.game_over() {
                    "Final"
                } else if context.started() {
                    "In Progress"
                } else {
                    "Pending"
                };
                writeln!(
                    &mut tw, "{}\t{}\t{}\t{}\t{}\t{}",
                    matchup_index,
                    away_team, context.away_score(),
                    home_team, context.home_score(),
                    status
                ).map_err(|e| e.to_string())?;
            }
            tw.flush().map_err(|e| e.to_string())?;
            println!();
        }
    }

    // Display champion if playoffs are complete
    if playoffs.complete() {
        if let Some(champion_id) = playoffs.champion() {
            let champion = season.team(champion_id).unwrap();
            println!("Champion: {}", champion.name());
        }
    } else {
        println!("Playoffs in progress");
    }

    Ok(())
}
