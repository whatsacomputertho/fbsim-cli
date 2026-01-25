use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;

use crate::cli::league::season::playoffs::round::FbsimLeagueSeasonPlayoffsRoundGetArgs;

use serde_json;
use tabwriter::TabWriter;

pub fn get_playoffs_round(args: FbsimLeagueSeasonPlayoffsRoundGetArgs) -> Result<(), String> {
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
    let rounds = playoffs.rounds();

    if rounds.is_empty() {
        return Err(format!("Playoffs have not been generated for the {} season", args.year));
    }

    // Get the round
    let round = match rounds.get(args.round) {
        Some(r) => r,
        None => return Err(format!("No playoff round found with index: {}", args.round)),
    };

    // Display the round
    println!("Playoff Round {} - {} Season", args.round, args.year);
    let mut tw = TabWriter::new(stdout());
    writeln!(&mut tw, "Matchup\tAway Team\tAway Score\tHome Team\tHome Score\tStatus").map_err(|e| e.to_string())?;
    for (matchup_idx, matchup) in round.matchups().iter().enumerate() {
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
            matchup_idx,
            away_team, context.away_score(),
            home_team, context.home_score(),
            status
        ).map_err(|e| e.to_string())?;
    }
    tw.flush().map_err(|e| e.to_string())?;

    Ok(())
}
