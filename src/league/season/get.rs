use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;
use fbsim_core::league::season::LeagueSeason;

use crate::cli::league::season::FbsimLeagueSeasonGetArgs;

use serde_json;
use tabwriter::TabWriter;

pub fn get_season(args: FbsimLeagueSeasonGetArgs) -> Result<(), String> {
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

    // Get a season from the league
    let season: &LeagueSeason = match league.season(args.year) {
        Some(season) => season,
        None => return Err(format!("No season found with year: {}", args.year)),
    };

    // Check if teams exist
    if season.teams().is_empty() {
        return Err(format!("No teams have been added to the {} season yet", args.year));
    }

    // Display the season teams in a table
    let mut tw = TabWriter::new(stdout());
    writeln!(&mut tw,"Team\tRecord").map_err(|e| e.to_string())?;
    for (id, team) in season.teams().iter() {
        let matchups = season.team_matchups(*id)?;
        writeln!(
            &mut tw, "{}\t{}",
            team.name(), matchups.record()
        ).map_err(|e| e.to_string())?;
    }

    // Display the season weeks in a table if schedule exists
    if !season.weeks().is_empty() {
        writeln!(&mut tw,"\nWeek\tGames\tSimulated").map_err(|e| e.to_string())?;
        for (i, week) in season.weeks().iter().enumerate() {
            writeln!(
                &mut tw, "{}\t{}\t{}", i+1,
                week.matchups().len(),
                week.matchups().iter().filter(|m| m.context().game_over()).collect::<Vec<_>>().len()
            ).map_err(|e| e.to_string())?;
        }
    }

    // Display playoff information
    let playoffs = season.playoffs();
    if playoffs.started() {
        writeln!(&mut tw, "\nPlayoffs ({} teams)", playoffs.num_teams()).map_err(|e| e.to_string())?;

        // Display conference brackets
        for (conf_index, rounds) in playoffs.conference_brackets() {
            let conf_name = season.conferences().get(*conf_index)
                .map(|c| c.name().to_string())
                .unwrap_or_else(|| format!("Conference {}", conf_index));
            writeln!(&mut tw, "\n{}", conf_name).map_err(|e| e.to_string())?;
            writeln!(&mut tw, "Round\tMatchups\tSimulated").map_err(|e| e.to_string())?;
            for (i, round) in rounds.iter().enumerate() {
                let simulated = round.matchups().iter().filter(|m| m.context().game_over()).count();
                writeln!(
                    &mut tw, "{}\t{}\t{}",
                    i, round.matchups().len(), simulated
                ).map_err(|e| e.to_string())?;
            }
        }

        // Display winners bracket
        let winners = playoffs.winners_bracket();
        if !winners.is_empty() {
            writeln!(&mut tw, "\nChampionship Bracket").map_err(|e| e.to_string())?;
            writeln!(&mut tw, "Round\tMatchups\tSimulated").map_err(|e| e.to_string())?;
            for (i, round) in winners.iter().enumerate() {
                let simulated = round.matchups().iter().filter(|m| m.context().game_over()).count();
                writeln!(
                    &mut tw, "{}\t{}\t{}",
                    i, round.matchups().len(), simulated
                ).map_err(|e| e.to_string())?;
            }
        }

        // Display champion if playoffs are complete
        if playoffs.complete() {
            if let Some(champion_id) = playoffs.champion() {
                let champion = season.team(champion_id).unwrap();
                writeln!(&mut tw, "\nChampion: {}", champion.name()).map_err(|e| e.to_string())?;
            }
        } else {
            writeln!(&mut tw, "\nPlayoffs in progress").map_err(|e| e.to_string())?;
        }
    }

    tw.flush().map_err(|e| e.to_string())?;
    Ok(())
}
