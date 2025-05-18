use std::fs;

use fbsim_core::league::League;

use crate::cli::league::season::FbsimLeagueSeasonCreateArgs;

use serde_json;

pub fn create_season(args: FbsimLeagueSeasonCreateArgs) {
    // Load the league from its file as mutable
    let mut league: League = serde_json::from_str(
        &fs::read_to_string(&args.league).unwrap()
    ).unwrap();

    // Create a season for the league
    let _ = league.create_season();

    // Serialize the league as JSON
    let league_str: String = serde_json::to_string_pretty(&league).unwrap();

    // Write the league back to its file
    _ = fs::write(&args.league, league_str);
}
