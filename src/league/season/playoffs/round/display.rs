use std::io::{Write, stdout};

use fbsim_core::league::season::LeagueSeason;
use tabwriter::TabWriter;

pub fn display_traditional_round(
    season: &LeagueSeason,
    round_index: usize,
    year: usize
) -> Result<(), String> {
    let playoffs = season.playoffs();
    let brackets = playoffs.conference_brackets();

    if brackets.is_empty() {
        return Err(format!("Playoffs have not been generated for the {} season", year));
    }

    // For non-conference playoffs, use the first (only) bracket
    let rounds = match brackets.values().next() {
        Some(r) => r,
        None => return Err(format!("No playoff bracket found for the {} season", year)),
    };
    let round = match rounds.get(round_index) {
        Some(r) => r,
        None => return Err(format!("No playoff round found with ID: {}", round_index)),
    };

    // Display the round
    println!("=== Playoff Round {} ===", round_index);
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
    Ok(())
}

pub fn display_conference_round(
    season: &LeagueSeason,
    round_index: usize,
    filter_conference: Option<usize>,
    year: usize
) -> Result<(), String> {
    let playoffs = season.playoffs();
    let conference_brackets = playoffs.conference_brackets();
    let conferences = season.conferences();
    let num_conferences = conferences.len();
    if conference_brackets.is_empty() {
        return Err(format!("Conference playoffs have not been generated for the {} season", year));
    }

    for (conf_index, conf_rounds) in conference_brackets.iter() {
        // Skip if filtering to specific conference
        if let Some(filter) = filter_conference {
            if filter != *conf_index {
                continue;
            }
        }

        // Get the number of conference rounds
        let conf_name = conferences.get(*conf_index)
            .map(|c| c.name().to_string())
            .unwrap_or_else(|| format!("Conference {}", conf_index));
        if let Some(round) = conf_rounds.get(round_index) {
            println!("=== {} Conference Playoffs ===", conf_name);
            println!("--- Round {} ---", round_index);
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
        } else if filter_conference.is_some() {
            return Err(format!("No round {} found for conference {}", round_index, conf_name));
        } else {
            return Err(format!("No round {} found in conference playoffs", round_index));
        }
        if *conf_index != (num_conferences - 1) {
            println!();
        }
    }
    Ok(())
}

pub fn display_winners_bracket_round(
    season: &LeagueSeason,
    round_index: usize,
    year: usize
) -> Result<(), String> {
    let playoffs = season.playoffs();
    let winners_bracket = playoffs.winners_bracket();
    if winners_bracket.is_empty() {
        return Err(format!("Winners' bracket has not been generated for the {} season", year));
    }
    if let Some(round) = winners_bracket.get(round_index) {
        println!("=== Championship Bracket ===");
        println!("--- Round {} ---", round_index);
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
    } else {
        return Err(format!("No round {} found in winners' bracket", round_index));
    }
    Ok(())
}
