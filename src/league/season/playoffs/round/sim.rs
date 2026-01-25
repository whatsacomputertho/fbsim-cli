use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;

use crate::cli::league::season::playoffs::round::FbsimLeagueSeasonPlayoffsRoundSimArgs;

use serde_json;
use tabwriter::TabWriter;

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

    // Simulate the playoff round
    let mut rng = rand::thread_rng();
    if let Err(e) = season.sim_playoff_round(args.round, &mut rng) {
        return Err(format!("Failed to simulate playoff round: {}", e));
    }

    // Generate the next round if the current round is complete and playoffs are not done
    let playoffs = season.playoffs();
    let round_complete = playoffs.rounds().get(args.round).map(|r| r.complete()).unwrap_or(false);
    let playoffs_complete = playoffs.complete();

    if round_complete && !playoffs_complete {
        if let Err(e) = season.generate_next_playoff_round(&mut rng) {
            return Err(format!("Failed to generate next playoff round: {}", e));
        }
    }

    // Get the year for display
    let year = *season.year();

    // Display results
    let round = match season.playoffs().rounds().get(args.round) {
        Some(r) => r,
        None => return Err(format!("No playoff round found with index: {}", args.round)),
    };

    println!("Playoff Round {} Results", args.round);
    let mut tw = TabWriter::new(stdout());
    writeln!(&mut tw, "Matchup\tAway Team\tAway Score\tHome Team\tHome Score").map_err(|e| e.to_string())?;
    for (matchup_idx, matchup) in round.matchups().iter().enumerate() {
        let away_team = season.team(*matchup.away_team()).unwrap().name();
        let home_team = season.team(*matchup.home_team()).unwrap().name();
        let context = matchup.context();
        writeln!(
            &mut tw, "{}\t{}\t{}\t{}\t{}",
            matchup_idx,
            away_team, context.away_score(),
            home_team, context.home_score()
        ).map_err(|e| e.to_string())?;
    }
    tw.flush().map_err(|e| e.to_string())?;

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

    let _ = year; // suppress unused warning
    Ok(())
}
