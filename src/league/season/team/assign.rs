use std::fs;

use fbsim_core::league::League;

use crate::cli::league::season::team::FbsimLeagueSeasonTeamAssignArgs;

use serde_json;

pub fn assign_team(args: FbsimLeagueSeasonTeamAssignArgs) -> Result<(), String> {
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

    // Verify team exists
    let team_name = match season.team(args.team) {
        Some(t) => t.name().to_string(),
        None => return Err(format!("No team found with ID: {}", args.team)),
    };

    // Verify conference exists and get info
    let conf_name = match season.conference(args.conference) {
        Some(c) => c.name().to_string(),
        None => return Err(format!("No conference found with index: {}", args.conference)),
    };

    // Verify division exists and get info
    let div_name = match season.conference(args.conference).and_then(|c| c.division(args.division)) {
        Some(d) => d.name().to_string(),
        None => return Err(format!("No division found with ID: {}", args.division)),
    };

    // Verify the team is not already assigned to a division in any conference
    for (ci, conference) in season.conferences().iter().enumerate() {
        for (di, division) in conference.divisions().iter().enumerate() {
            if division.teams().contains(&args.team) {
                return Err(format!(
                    "{} is already assigned to {} {} (conference {}, division {})",
                    team_name, conference.name(), division.name(), ci, di
                ));
            }
        }
    }

    // Assign the team to the division
    let conference = season.conference_mut(args.conference).unwrap();
    let division = conference.division_mut(args.division).unwrap();
    division.add_team(args.team)?;

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
    println!("{} assigned to {} {}", team_name, conf_name, div_name);
    Ok(())
}
