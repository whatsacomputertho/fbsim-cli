use std::fs;
use std::io::{Write, stdout};
use std::collections::BTreeMap;

use fbsim_core::game::stat::OffensiveStats;
use fbsim_core::league::League;
use fbsim_core::league::team::LeagueTeam;
use fbsim_core::league::matchup::LeagueMatchups;

use crate::cli::league::team::stats::FbsimLeagueTeamStatsPassingArgs;

use serde_json;
use tabwriter::TabWriter;

pub fn list_passing(args: FbsimLeagueTeamStatsPassingArgs) -> Result<(), String> {
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

    // Get the collection of teams from the league
    let teams: &BTreeMap<usize, LeagueTeam> = league.teams();
    for (id, _) in teams.iter() {
        let matchups: LeagueMatchups = league.team_matchups(*id);

        // Get the most recent team name
        let team = if !matchups.matchups().is_empty() {
            if let Some((year, _)) = matchups.matchups().iter().next_back() {
                league.season(*year).unwrap().team(*id).unwrap().name()
            } else {
                "(No Name)"
            }
        } else {
            "(No Name)"
        };

        // Get the team stats and display them
        let stats: OffensiveStats = matchups.stats();
        let passing = stats.passing();
        let completions = passing.completions();
        let attempts = passing.attempts();
        let percent: f64 = completions as f64 / attempts as f64;
        writeln!(
            &mut tw, "{}\t{}\t{}/{}\t{:.2}%\t{}\t{}\t{}",
            id, team, completions, attempts, percent * 100.0,
            passing.yards(), passing.touchdowns(), passing.interceptions()
        ).map_err(|e| e.to_string())?;
    }
    tw.flush().map_err(|e| e.to_string())?;
    Ok(())
}
