use std::fs;

use fbsim_core::league::League;

use crate::cli::league::season::playoffs::round::FbsimLeagueSeasonPlayoffsRoundGetArgs;
use crate::league::season::playoffs::round::display;

use serde_json;

pub fn get_playoffs_round(args: FbsimLeagueSeasonPlayoffsRoundGetArgs) -> Result<(), String> {
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

    // Get the season and playoffs
    let season = match league.season(args.year) {
        Some(season) => season,
        None => return Err(format!("No season found with year: {}", args.year)),
    };
    let playoffs = season.playoffs();

    // Display the playoff round
    if playoffs.is_conference_playoff() {
        if args.winners_bracket {
            display::display_winners_bracket_round(season, args.round, args.year)
        } else {
            display::display_conference_round(season, args.round, args.conference, args.year)
        }
    } else {
        display::display_traditional_round(season, args.round, args.year)
    }
}
