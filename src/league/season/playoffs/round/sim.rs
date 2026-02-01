use std::fs;

use fbsim_core::league::League;
use fbsim_core::league::season::LeagueSeason;

use crate::cli::league::season::playoffs::round::FbsimLeagueSeasonPlayoffsRoundSimArgs;
use crate::league::season::playoffs::round::display;

use serde_json;

pub fn sim_playoffs_round(args: FbsimLeagueSeasonPlayoffsRoundSimArgs) -> Result<(), String> {
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

    // Get the current season
    let season = match league.current_season_mut() {
        Some(s) => s,
        None => return Err(String::from("No current season found")),
    };

    let mut rng = rand::thread_rng();
    let is_conference_playoff = season.playoffs().is_conference_playoff();
    let year = *season.year();

    // Determine the current round and whether it's a winners bracket round
    let (round_index, is_winners_bracket) = find_current_round(season)?;

    // Simulate the round
    if is_winners_bracket {
        if let Err(e) = season.sim_winners_bracket_round(round_index, &mut rng) {
            return Err(format!("Failed to simulate winners bracket round: {}", e));
        }
    } else if let Err(e) = season.sim_playoff_round(round_index, &mut rng) {
        return Err(format!("Failed to simulate playoff round: {}", e));
    }

    // Try to generate the next round if playoffs are not yet complete.
    if !season.playoffs().complete() {
        let _ = season.generate_next_playoff_round(&mut rng);
    }

    // Display results using the same format as the get command
    if is_winners_bracket {
        display::display_winners_bracket_round(season, round_index, year)?;
    } else if is_conference_playoff {
        display::display_conference_round(season, round_index, None, year)?;
    } else {
        display::display_traditional_round(season, round_index, year)?;
    }

    // Display champion if playoffs are complete
    if season.playoffs().complete() {
        if let Some(champion_id) = season.playoffs().champion() {
            let champion = season.team(champion_id).unwrap();
            println!("\nChampion: {}", champion.name());
        }
    }

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

/// Find the current incomplete round. Returns (round_index, is_winners_bracket).
fn find_current_round(
    season: &LeagueSeason
) -> Result<(usize, bool), String> {
    let playoffs = season.playoffs();

    if playoffs.complete() {
        return Err(String::from("Playoffs are already complete"));
    }

    // Check conference brackets first
    if !playoffs.conference_brackets_complete() {
        // Find the first incomplete round across conference brackets
        for (_conf_index, rounds) in playoffs.conference_brackets().iter() {
            for (round_index, round) in rounds.iter().enumerate() {
                if !round.complete() {
                    return Ok((round_index, false));
                }
            }
        }
        return Err(String::from("All conference playoff rounds are complete"));
    }

    // Conference brackets are complete, check winners bracket
    let winners_bracket = playoffs.winners_bracket();
    if winners_bracket.is_empty() {
        return Err(String::from("Winners bracket has not been generated yet"));
    }
    for (round_index, round) in winners_bracket.iter().enumerate() {
        if !round.complete() {
            return Ok((round_index, true));
        }
    }
    Err(String::from("All playoff rounds are complete"))
}
