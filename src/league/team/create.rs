use std::fs;

use fbsim_core::league::{League, LeagueTeam, LeagueSeasonTeam};

use crate::cli::league::team::FbsimLeagueTeamCreateArgs;

use serde_json;

pub fn create_team(args: FbsimLeagueTeamCreateArgs) -> Result<(), String> {
    // Load the league from its file as mutable
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

    // Get the league team corresponding to the given team
    let team_opt = league.team(args.id);
    let team: &LeagueTeam = match team_opt {
        Some(team) => team,
        None => return Err(format!("No team found with ID: {}", args.id)),
    };

    // Create the league season team from the given parameters
    let season_team = LeagueSeasonTeam::new(args.id, args.name, args.logo, args.offense, args.defense, team);

    // Serialize the league season team as JSON
    let season_team_str_res = serde_json::to_string_pretty(&season_team);
    let season_team_str: String = match season_team_str_res {
        Ok(season_team_str) => season_team_str,
        Err(error) => return Err(format!("Error serializing season team: {}", error)),
    };

    // Write the league season team back to its output file
    let write_res = fs::write(&args.file, season_team_str);
    let _ = match write_res {
        Ok(()) => (),
        Err(error) => return Err(format!("Error writing season team file: {}", error)),
    };
    Ok(())
}
