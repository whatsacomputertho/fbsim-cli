use std::fs;

use fbsim_core::league::League;

use crate::cli::league::season::week::matchup::FbsimLeagueSeasonWeekMatchupGetArgs;

use serde_json;

pub fn get_matchup(args: FbsimLeagueSeasonWeekMatchupGetArgs) -> Result<(), String> {
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

    // Get the league season
    let season = match league.season(args.year) {
        Some(season) => season,
        None => return Err(format!("No season found with year: {}", args.year)),
    };

    // Get the league season week from the league season
    let week = match season.weeks().get(args.week) {
        Some(week) => week,
        None => return Err(format!("No week found in season {} with id: {}", args.year, args.week)),
    };

    // Get the league season matchup from the league week
    let matchup = match week.matchups().get(args.matchup) {
        Some(matchup) => matchup,
        None => return Err(
            format!(
                "No matchup found in season {} week {} with id: {}",
                args.year, args.week, args.matchup
            )
        ),
    };

    // Get the team names
    let away_team = season.team(*matchup.away_team()).unwrap().name();
    let home_team = season.team(*matchup.home_team()).unwrap().name();

    // Get the game context and stats
    let context = matchup.context();

    // Display based on game state
    println!("{} season week {} matchup {}", args.year, args.week, args.matchup);
    println!();
    println!("{} @ {}", away_team, home_team);
    println!();

    if context.game_over() {
        println!("{} Final", context);

        if let Some(home_stats) = matchup.home_stats() {
            println!();
            println!("{} stats\n{}", context.home_team_short(), home_stats);
        }
        if let Some(away_stats) = matchup.away_stats() {
            println!();
            println!("{} stats\n{}", context.away_team_short(), away_stats);
        }
    } else if context.started() {
        // Display play-by-play log up to this point
        if let Some(game) = matchup.game() {
            for drive in game.drives().iter() {
                println!("{}\n", drive);
            }
        } else {
            println!("{}", context);
        }
    } else {
        println!("{} Pending", context);
    }

    Ok(())
}
