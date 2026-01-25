use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;

use crate::cli::league::season::week::matchup::FbsimLeagueSeasonWeekMatchupGetArgs;

use serde_json;
use tabwriter::TabWriter;

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

    // TODO: Calculate a summary of the matchup
    // TODO: Display the summary in a nice looking way (not JSON)

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

    // Display the matchup in a table
    let mut tw = TabWriter::new(stdout());
    writeln!(&mut tw,"Away Team\tAway Score\tHome Team\tHome Score").map_err(|e| e.to_string())?;
    writeln!(
        &mut tw,"{}\t{}\t{}\t{}",
        away_team, context.away_score(),
        home_team, context.home_score()
    ).map_err(|e| e.to_string())?;
    tw.flush().map_err(|e| e.to_string())?;

    // Get the home and away stats
    let home_stats_opt = matchup.home_stats();
    let away_stats_opt = matchup.away_stats();

    // Get the game
    let game_opt = matchup.game();

    // Display the passing stats if the game is complete or in-progress
    writeln!(&mut tw).map_err(|e| e.to_string())?;
    writeln!(
        &mut tw,
        "Team\tCompletions\tComp %\tYards\tTouchdowns\tInterceptions"
    ).map_err(|e| e.to_string())?;
    if let Some(home_stats) = home_stats_opt {
        let passing = home_stats.passing();
        let completions = passing.completions();
        let attempts = passing.attempts();
        let percent: f64 = completions as f64 / attempts as f64;
        writeln!(
            &mut tw, "{}\t{}/{}\t{:.2}%\t{}\t{}\t{}",
            home_team, completions, attempts, percent * 100.0,
            passing.yards(), passing.touchdowns(), passing.interceptions()
        ).map_err(|e| e.to_string())?;
    } else if let Some(game) = game_opt {
        let home_stats = game.home_stats();
        let passing = home_stats.passing();
        let completions = passing.completions();
        let attempts = passing.attempts();
        let percent: f64 = completions as f64 / attempts as f64;
        writeln!(
            &mut tw, "{}\t{}/{}\t{:.2}%\t{}\t{}\t{}",
            home_team, completions, attempts, percent * 100.0,
            passing.yards(), passing.touchdowns(), passing.interceptions()
        ).map_err(|e| e.to_string())?;
    } else {
        let status = if context.started() { "In Progress" } else { "Pending" };
        writeln!(&mut tw, "{}\t{}", home_team, status).map_err(|e| e.to_string())?;
    }
    if let Some(away_stats) = away_stats_opt {
        let passing = away_stats.passing();
        let completions = passing.completions();
        let attempts = passing.attempts();
        let percent: f64 = completions as f64 / attempts as f64;
        writeln!(
            &mut tw, "{}\t{}/{}\t{:.2}%\t{}\t{}\t{}",
            away_team, completions, attempts, percent * 100.0,
            passing.yards(), passing.touchdowns(), passing.interceptions()
        ).map_err(|e| e.to_string())?;
    } else if let Some(game) = game_opt {
        let away_stats = game.away_stats();
        let passing = away_stats.passing();
        let completions = passing.completions();
        let attempts = passing.attempts();
        let percent: f64 = completions as f64 / attempts as f64;
        writeln!(
            &mut tw, "{}\t{}/{}\t{:.2}%\t{}\t{}\t{}",
            away_team, completions, attempts, percent * 100.0,
            passing.yards(), passing.touchdowns(), passing.interceptions()
        ).map_err(|e| e.to_string())?;
    } else {
        let status = if context.started() { "In Progress" } else { "Pending" };
        writeln!(&mut tw, "{}\t{}", away_team, status).map_err(|e| e.to_string())?;
    }
    tw.flush().map_err(|e| e.to_string())?;

    // Display the rushing stats
    writeln!(&mut tw).map_err(|e| e.to_string())?;
    writeln!(
        &mut tw,
        "Team\tRushes\tYards\tYPC\tTouchdowns\tFumbles"
    ).map_err(|e| e.to_string())?;
    if let Some(home_stats) = home_stats_opt {
        let rushing = home_stats.rushing();
        let rushes = rushing.rushes();
        let yards = rushing.yards();
        let ypc: f64 = yards as f64 / rushes as f64;
        writeln!(
            &mut tw, "{}\t{}\t{}\t{:.2}\t{}\t{}",
            home_team, rushes, yards, ypc,
            rushing.touchdowns(), rushing.fumbles()
        ).map_err(|e| e.to_string())?;
    } else if let Some(game) = game_opt {
        let home_stats = game.home_stats();
        let rushing = home_stats.rushing();
        let rushes = rushing.rushes();
        let yards = rushing.yards();
        let ypc: f64 = yards as f64 / rushes as f64;
        writeln!(
            &mut tw, "{}\t{}\t{}\t{:.2}\t{}\t{}",
            home_team, rushes, yards, ypc,
            rushing.touchdowns(), rushing.fumbles()
        ).map_err(|e| e.to_string())?;
    } else {
        let status = if context.started() { "In Progress" } else { "Pending" };
        writeln!(&mut tw, "{}\t{}", home_team, status).map_err(|e| e.to_string())?;
    }
    if let Some(away_stats) = away_stats_opt {
        let rushing = away_stats.rushing();
        let rushes = rushing.rushes();
        let yards = rushing.yards();
        let ypc: f64 = yards as f64 / rushes as f64;
        writeln!(
            &mut tw, "{}\t{}\t{}\t{:.2}\t{}\t{}",
            away_team, rushes, yards, ypc,
            rushing.touchdowns(), rushing.fumbles()
        ).map_err(|e| e.to_string())?;
    } else if let Some(game) = game_opt {
        let away_stats = game.away_stats();
        let rushing = away_stats.rushing();
        let rushes = rushing.rushes();
        let yards = rushing.yards();
        let ypc: f64 = yards as f64 / rushes as f64;
        writeln!(
            &mut tw, "{}\t{}\t{}\t{:.2}\t{}\t{}",
            away_team, rushes, yards, ypc,
            rushing.touchdowns(), rushing.fumbles()
        ).map_err(|e| e.to_string())?;
    } else {
        let status = if context.started() { "In Progress" } else { "Pending" };
        writeln!(&mut tw, "{}\t{}", away_team, status).map_err(|e| e.to_string())?;
    }
    tw.flush().map_err(|e| e.to_string())?;
    Ok(())
}
