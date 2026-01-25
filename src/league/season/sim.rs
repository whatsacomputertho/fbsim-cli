use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;

use crate::cli::league::season::FbsimLeagueSeasonSimArgs;

use serde_json;
use tabwriter::TabWriter;

pub fn sim_season(args: FbsimLeagueSeasonSimArgs) -> Result<(), String> {
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

    // Validate that the season has teams and a schedule
    let season = match league.current_season() {
        Some(s) => s,
        None => return Err(String::from("No current season found")),
    };
    if season.teams().is_empty() {
        return Err(String::from("No teams have been added to the current season yet"));
    }
    if season.weeks().is_empty() {
        return Err(String::from("No schedule has been generated for the current season yet"));
    }

    // Simulate the current league season
    let mut rng = rand::thread_rng();
    if let Err(e) = league.sim(&mut rng) {
        return Err(
            format!(
                "Failed to simulate current season: {}",
                e
            )
        );
    }

    // Display final standings
    let season = match league.current_season() {
        Some(s) => s,
        None => return Err(String::from("No current season found")),
    };

    println!("{} season final standings", season.year());
    println!();
    let mut tw = TabWriter::new(stdout());
    writeln!(&mut tw, "Rank\tTeam\tRecord").map_err(|e| e.to_string())?;
    for (rank, (team_id, _win_pct)) in season.standings().iter().enumerate() {
        let team = season.team(*team_id).unwrap();
        let record = season.team_matchups(*team_id)?.record();
        writeln!(
            &mut tw, "{}\t{}\t{}",
            rank + 1,
            team.name(),
            record
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
