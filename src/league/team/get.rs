use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;
use fbsim_core::league::matchup::LeagueMatchups;

use crate::cli::league::team::FbsimLeagueTeamGetArgs;

use serde_json;
use tabwriter::TabWriter;

pub fn get_team(args: FbsimLeagueTeamGetArgs) -> Result<(), String> {
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

    // TODO: Get the team's most recent name

    // Calculate the team's all-time record
    let matchups: LeagueMatchups = league.team_matchups(args.team);
    let mut tw = TabWriter::new(stdout());
    write!(&mut tw, "Year\tRecord\n").map_err(|e| e.to_string())?;

    // Calculate the team's record for each previous season
    for (year, season) in matchups.matchups().iter() {
        write!(&mut tw, "{}\t{}\n", year, season.record()).map_err(|e| e.to_string())?;
    }
    write!(&mut tw, "Total\t{}\n", matchups.record()).map_err(|e| e.to_string())?;
    tw.flush().map_err(|e| e.to_string())?;
    Ok(())
}
