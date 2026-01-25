use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;
use fbsim_core::league::matchup::LeagueTeamRecord;

use crate::cli::league::season::team::FbsimLeagueSeasonTeamGetArgs;

use serde_json;
use tabwriter::TabWriter;

pub fn get_season_team(args: FbsimLeagueSeasonTeamGetArgs) -> Result<(), String> {
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

    // Get the league season team from the league season
    let team = match season.team(args.id) {
        Some(team) => team,
        None => return Err(format!("No team found in season {} with id: {}", args.year, args.id)),
    };

    // Get the league season team's matchups from the league season
    let matchups = season.team_matchups(args.id)?;

    // Get playoff information
    let playoffs = season.playoffs();
    let playoff_record = playoffs.record(args.id);
    let playoffs_started = playoffs.started();
    let playoffs_complete = playoffs.complete();
    let is_champion = playoffs_complete && playoffs.champion() == Some(args.id);

    // Check if team was in playoffs by looking for any playoff matchups
    let mut in_playoffs = false;
    for round in playoffs.rounds().iter() {
        for matchup in round.matchups().iter() {
            if *matchup.home_team() == args.id || *matchup.away_team() == args.id {
                in_playoffs = true;
                break;
            }
        }
        if in_playoffs {
            break;
        }
    }

    // Calculate total record (regular season + playoffs)
    let regular_record = matchups.record();
    let mut total_record = LeagueTeamRecord::new();
    total_record.increment_wins(*regular_record.wins() + *playoff_record.wins());
    total_record.increment_losses(*regular_record.losses() + *playoff_record.losses());
    total_record.increment_ties(*regular_record.ties() + *playoff_record.ties());

    // Display the results in a table
    let mut tw = TabWriter::new(stdout());
    writeln!(&mut tw, "Team:\t{}", team.name()).map_err(|e| e.to_string())?;
    writeln!(&mut tw, "Record:\t{}", total_record).map_err(|e| e.to_string())?;

    // Display playoff record only if playoffs have started
    if playoffs_started {
        if !in_playoffs {
            writeln!(&mut tw, "Playoff Record:\tN/A").map_err(|e| e.to_string())?;
        } else if *playoff_record.wins() > 0 || *playoff_record.losses() > 0 || *playoff_record.ties() > 0 {
            writeln!(&mut tw, "Playoff Record:\t{}", playoff_record).map_err(|e| e.to_string())?;
        }
    }

    // Display champion status only if playoffs are complete
    if playoffs_complete && is_champion {
        writeln!(&mut tw, "Champion:\tYes").map_err(|e| e.to_string())?;
    }

    writeln!(&mut tw).map_err(|e| e.to_string())?;

    // Display each regular season matchup
    writeln!(
        &mut tw,
        "Week\tHome Team\tHome Score\tAway Team\tAway Score"
    ).map_err(|e| e.to_string())?;
    for (i, matchup) in matchups.matchups().iter().enumerate() {
        match matchup {
            Some(m) => {
                let context = m.context();
                let away_team = season.team(*m.away_team()).unwrap().name();
                let home_team = season.team(*m.home_team()).unwrap().name();
                writeln!(
                    &mut tw, "{}\t{}\t{}\t{}\t{}", i+1,
                    home_team, context.home_score(),
                    away_team, context.away_score()
                ).map_err(|e| e.to_string())?;
            },
            None => {
                writeln!(
                    &mut tw, "{}\tBYE", i+1
                ).map_err(|e| e.to_string())?;
            },
        }
    }

    // Display playoff matchups if the team participated
    let rounds = playoffs.rounds();
    let mut has_playoff_matchups = false;
    for round in rounds.iter() {
        for matchup in round.matchups().iter() {
            if *matchup.home_team() == args.id || *matchup.away_team() == args.id {
                has_playoff_matchups = true;
                break;
            }
        }
        if has_playoff_matchups {
            break;
        }
    }

    if has_playoff_matchups {
        writeln!(&mut tw, "\nPlayoffs").map_err(|e| e.to_string())?;
        writeln!(
            &mut tw,
            "Round\tHome Team\tHome Score\tAway Team\tAway Score"
        ).map_err(|e| e.to_string())?;

        for (round_id, round) in rounds.iter().enumerate() {
            for matchup in round.matchups().iter() {
                if *matchup.home_team() == args.id || *matchup.away_team() == args.id {
                    let context = matchup.context();
                    let away_team = season.team(*matchup.away_team()).unwrap().name();
                    let home_team = season.team(*matchup.home_team()).unwrap().name();
                    writeln!(
                        &mut tw, "{}\t{}\t{}\t{}\t{}",
                        round_id,
                        home_team, context.home_score(),
                        away_team, context.away_score()
                    ).map_err(|e| e.to_string())?;
                }
            }
        }
    }

    tw.flush().map_err(|e| e.to_string())?;
    Ok(())
}
