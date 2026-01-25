use std::fs;
use std::io::{Write, stdout};
use std::collections::BTreeMap;

use fbsim_core::league::League;
use fbsim_core::league::team::LeagueTeam;
use fbsim_core::league::matchup::{LeagueMatchups, LeagueTeamRecord};

use crate::cli::league::team::FbsimLeagueTeamListArgs;

use serde_json;
use tabwriter::TabWriter;

pub fn list_teams(args: FbsimLeagueTeamListArgs) -> Result<(), String> {
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
    writeln!(&mut tw, "ID\tTeam\tSeasons\tRecord\tChamp. Apps\tChampionships").map_err(|e| e.to_string())?;

    // Get the collection of teams from the league
    let teams: &BTreeMap<usize, LeagueTeam> = league.teams();
    for (id, _) in teams.iter() {
        let matchups: LeagueMatchups = league.team_matchups(*id)?;

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

        // Start with regular season record
        let regular_season_record = matchups.record();
        let mut total_record = LeagueTeamRecord::new();
        total_record.increment_wins(*regular_season_record.wins());
        total_record.increment_losses(*regular_season_record.losses());
        total_record.increment_ties(*regular_season_record.ties());

        // Add playoff record and calculate championship stats across all seasons
        let mut championship_appearances: usize = 0;
        let mut championships: usize = 0;

        for (year, _) in matchups.matchups().iter() {
            if let Some(season) = league.season(*year) {
                let playoffs = season.playoffs();
                if let Ok(season_playoff_record) = playoffs.record(*id) {
                    total_record.increment_wins(*season_playoff_record.wins());
                    total_record.increment_losses(*season_playoff_record.losses());
                    total_record.increment_ties(*season_playoff_record.ties());
                }

                if let Ok(true) = playoffs.in_championship(*id) {
                    championship_appearances += 1;
                }

                if let Some(champion_id) = playoffs.champion() {
                    if champion_id == *id {
                        championships += 1;
                    }
                }
            }
        }

        writeln!(
            &mut tw, "{}\t{}\t{}\t{}\t{}\t{}",
            id, team, matchups.matchups().len(), total_record,
            championship_appearances, championships
        ).map_err(|e| e.to_string())?;
    }
    tw.flush().map_err(|e| e.to_string())?;
    Ok(())
}
