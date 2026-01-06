use std::fs;

use fbsim_core::team::FootballTeam;
use fbsim_core::league::League;

use crate::cli::league::season::team::FbsimLeagueSeasonTeamAddArgs;

use serde_json;

pub fn add_season_team(args: FbsimLeagueSeasonTeamAddArgs) -> Result<(), String> {
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

    // Load the team from its file
    let season_team_file_res = &fs::read_to_string(&args.team);
    let season_team_file = match season_team_file_res {
        Ok(file) => file,
        Err(e) => return Err(format!("Error loading team file: {}", e)),
    };
    let season_team: FootballTeam = match serde_json::from_str(season_team_file) {
        Ok(team) => team,
        Err(e) => return Err(format!("Error loading team: {}", e)),
    };

    // Add the team to the season
    if let Err(e) = league.add_season_team(args.id, season_team) {
        return Err(format!("Failed to add team to season: {}", e));
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
