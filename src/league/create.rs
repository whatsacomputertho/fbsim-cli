use std::fs;
use std::str::FromStr;

use fbsim_core::league::League;

use crate::cli::league::FbsimLeagueCreateArgs;
use crate::cli::output::OutputFormat;

use serde_json;

pub fn create_league(args: FbsimLeagueCreateArgs) {
    // Instantiate a new league
    let league = League::new();

    // Serialize the league as a string based on the given output format
    let output_format = OutputFormat::from_str(
        &args.output_format.clone().unwrap_or(String::from(""))
    ).unwrap();
    let league_str: String = match output_format {
        OutputFormat::Json => {
            serde_json::to_string_pretty(&league).unwrap()
        },
        OutputFormat::Default => {
            format!("Not yet implemented")
            //format!("{}", league)
        }
    };

    // Write the league either to stdout or to a file
    match &args.output_file {
        Some(x) => {
            // Write the output to the output file
            _ = fs::write(x, league_str);
        },
        None => {
            // Print the output to stdout
            println!("{}", league_str);
        }
    }
}
