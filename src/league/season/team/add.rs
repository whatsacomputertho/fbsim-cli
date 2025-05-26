use std::fs;

use fbsim_core::league::League;
use fbsim_core::league::season::LeagueSeasonTeam;

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

    // Create the league season team and add to the season
    let season_team = LeagueSeasonTeam::new(args.name, args.logo, args.offense, args.defense);
    match league.add_season_team(args.id, season_team) {
        Ok(()) => (),
        Err(error) => return Err(format!("Failed to add team to season: {}", error)),
    }

    // Serialize the league as JSON
    let league_res = serde_json::to_string_pretty(&league);
    let league_str: String = match league_res {
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
