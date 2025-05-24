use std::fs;

use fbsim_core::league::{League, LeagueSeasonTeam};

use crate::cli::league::season::FbsimLeagueSeasonCreateArgs;

use serde_json;

pub fn create_season(args: FbsimLeagueSeasonCreateArgs) -> Result<(), String> {
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

    // Load the league teams from their subdirectory
    let mut teams = Vec::new();
    let team_dir_res = fs::read_dir(&args.teams);
    let team_dir = match team_dir_res {
        Ok(team_dir) => team_dir,
        Err(error) => return Err(format!("Error loading team directory: {}", error)),
    };
    for team_entry in team_dir {
        match team_entry {
            Ok(team_entry) => {
                // Load the team from its file and append to the list
                let team_file_res = &fs::read_to_string(team_entry.path());
                let team_file = match team_file_res {
                    Ok(file) => file,
                    Err(error) => return Err(format!("Error loading team file: {}", error)),
                };
                let team_res = serde_json::from_str(team_file);
                let team: LeagueSeasonTeam = match team_res {
                    Ok(team) => team,
                    Err(error) => return Err(format!("Error loading team from file: {}", error)),
                };
                teams.push(team);
            },
            Err(error) => return Err(format!("Error loading team file: {}", error)),
        }
    }

    // Create a season for the league
    let season_res = league.create_season(teams);
    let _ = match season_res {
        Ok(()) => (),
        Err(error) => return Err(format!("Error creating season: {}", error)),
    };

    // Serialize the league as JSON
    let league_str_res = serde_json::to_string_pretty(&league);
    let league_str: String = match league_str_res {
        Ok(league_str) => league_str,
        Err(error) => return Err(format!("Error serializing league: {}", error)),
    };

    // Write the league back to its file
    let write_res = fs::write(&args.league, league_str);
    let _ = match write_res {
        Ok(()) => (),
        Err(error) => return Err(format!("Error writing league file: {}", error)),
    };
    Ok(())
}
