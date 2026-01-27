use std::fs;

use fbsim_core::league::League;

use crate::cli::league::season::playoffs::FbsimLeagueSeasonPlayoffsGenArgs;

use serde_json;

pub fn gen_playoffs(args: FbsimLeagueSeasonPlayoffsGenArgs) -> Result<(), String> {
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

    // Generate the playoffs
    let mut rng = rand::thread_rng();

    let result_msg = if args.per_conference {
        // Validate conferences exist
        if season.conferences().is_empty() {
            return Err(String::from(
                "Per-conference playoffs (-p) require conferences to be defined. \
                Use 'league season conference add' first."
            ));
        }

        // Generate conference playoffs
        if let Err(e) = season.generate_playoffs_with_conferences(
            args.num_teams,
            args.division_winners,
            &mut rng
        ) {
            return Err(format!("Failed to generate playoffs: {}", e));
        }

        let num_conferences = season.conferences().len();
        format!(
            "Conference playoffs generated with {} teams per conference ({} conferences)",
            args.num_teams, num_conferences
        )
    } else {
        // Generate traditional playoffs
        if let Err(e) = season.generate_playoffs(args.num_teams, &mut rng) {
            return Err(format!("Failed to generate playoffs: {}", e));
        }

        format!("Playoffs generated with {} teams", args.num_teams)
    };

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

    println!("{}", result_msg);
    Ok(())
}
