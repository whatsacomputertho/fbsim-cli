use std::fs;
use std::io::{Write, stdout};

use fbsim_core::game::stat::OffensiveStats;
use fbsim_core::league::League;
use fbsim_core::league::season::matchup::LeagueSeasonMatchups;

use crate::cli::league::season::team::stats::FbsimLeagueSeasonTeamStatsPassingArgs;

use serde_json;
use tabwriter::TabWriter;

pub fn list_season_passing(args: FbsimLeagueSeasonTeamStatsPassingArgs) -> Result<(), String> {
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

    // Display the results in a table
    let mut tw = TabWriter::new(stdout());
    writeln!(
        &mut tw,
        "ID\tTeam\tCompletions\tComp %\tYards\tTouchdowns\tInterceptions"
    ).map_err(|e| e.to_string())?;

    // Get the league season
    let season = match league.season(args.year) {
        Some(season) => season,
        None => return Err(format!("No season found with year: {}", args.year)),
    };

    // Get the league season teams from the league season
    let teams = season.teams();
    for (id, team) in teams.iter() {
        let matchups: LeagueSeasonMatchups = season.team_matchups(*id)?;

        // Get the team stats and display them
        let stats: OffensiveStats = matchups.stats();
        let passing = stats.passing();
        let completions = passing.completions();
        let attempts = passing.attempts();
        let percent: f64 = completions as f64 / attempts as f64;
        writeln!(
            &mut tw, "{}\t{}\t{}/{}\t{:.2}%\t{}\t{}\t{}",
            id, team.name(), completions, attempts, percent * 100.0,
            passing.yards(), passing.touchdowns(), passing.interceptions()
        ).map_err(|e| e.to_string())?;
    }
    tw.flush().map_err(|e| e.to_string())?;
    Ok(())
}
