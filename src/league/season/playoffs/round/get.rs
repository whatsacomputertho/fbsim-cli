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

    if playoffs.is_conference_playoff() {
        display_conference_round(season, args.round, args.conference, args.year)?;
    } else {
        display_traditional_round(season, args.round, args.year)?;
    }

    Ok(())
}

fn display_traditional_round(
    season: &fbsim_core::league::season::LeagueSeason,
    round_index: usize,
    year: usize
) -> Result<(), String> {
    let playoffs = season.playoffs();
    let brackets = playoffs.conference_brackets();

    if brackets.is_empty() {
        return Err(format!("Playoffs have not been generated for the {} season", year));
    }

    // For traditional (non-conference) playoffs, use the first (only) bracket
    let rounds = match brackets.values().next() {
        Some(r) => r,
        None => return Err(format!("No playoff bracket found for the {} season", year)),
    };

    // Get the round
    let round = match rounds.get(round_index) {
        Some(r) => r,
        None => return Err(format!("No playoff round found with index: {}", round_index)),
    };

    // Display the round
    println!("Playoff Round {} - {} Season", round_index, year);
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

fn display_conference_round(
    season: &fbsim_core::league::season::LeagueSeason,
    round_index: usize,
    filter_conference: Option<usize>,
    year: usize
) -> Result<(), String> {
    let playoffs = season.playoffs();
    let conference_brackets = playoffs.conference_brackets();
    let conferences = season.conferences();

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

        let conf_name = conferences.get(*conf_index)
            .map(|c| c.name().to_string())
            .unwrap_or_else(|| format!("Conference {}", conf_index));

        if let Some(round) = conf_rounds.get(round_index) {
            println!("=== {} Round {} - {} Season ===", conf_name, round_index, year);
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
        }
    }

    // Also display winners bracket if it exists
    let winners_bracket = playoffs.winners_bracket();
    if !winners_bracket.is_empty() {
        println!();
        println!("=== Championship Bracket ===");
        for (bracket_round_index, round) in winners_bracket.iter().enumerate() {
            println!("Championship Round {}", bracket_round_index);
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
        }
    }

    Ok(())
}
