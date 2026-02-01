use std::io::{Write, stdout};

use fbsim_core::league::season::LeagueSeason;
use tabwriter::TabWriter;

pub fn display_playoffs(season: &LeagueSeason) -> Result<(), String> {
    let playoffs = season.playoffs();

    // Display conference brackets
    for (conf_index, rounds) in playoffs.conference_brackets() {
        let conf_name = season.conferences().get(*conf_index)
            .map(|c| c.name().to_string())
            .unwrap_or_else(|| format!("Conference {}", conf_index));
        println!("=== {} Conference Playoffs ===", conf_name);

        for (round_index, round) in rounds.iter().enumerate() {
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
            println!();
        }
    }

    // Display winners bracket
    let winners = playoffs.winners_bracket();
    if !winners.is_empty() {
        println!("=== Championship Bracket ===");
        for (round_index, round) in winners.iter().enumerate() {
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
