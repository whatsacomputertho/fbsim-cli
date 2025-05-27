use std::fs;

use fbsim_core::league::League;

use crate::cli::league::FbsimLeagueCreateArgs;

use serde_json;

pub fn create_league(args: FbsimLeagueCreateArgs) -> Result<(), String> {
    // Instantiate a new league
    let league = League::new();

    // Serialize the league as stringified JSON
    let league_str_res = serde_json::to_string_pretty(&league);
    let league_str = match league_str_res {
        Ok(league_str) => league_str,
        Err(error) => return Err(format!("Error serializing league: {}", error))
    };

    // Write the league to its output file
    let write_res = fs::write(args.output_file, league_str);
    let _ = match write_res {
        Ok(()) => (),
        Err(error) => return Err(format!("Error writing league file: {}", error)),
    };
    Ok(())
}
