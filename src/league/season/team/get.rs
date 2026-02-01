use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;
use fbsim_core::league::matchup::LeagueTeamRecord;
use fbsim_core::league::season::playoffs::picture::PlayoffStatus;

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

    // Get the league season and team
    let season = match league.season(args.year) {
        Some(season) => season,
        None => return Err(format!("No season found with year: {}", args.year)),
    };
    let team = match season.team(args.id) {
        Some(team) => team,
        None => return Err(format!("No team found in season {} with id: {}", args.year, args.id)),
    };

    // Get the team's matchups from the league season
    let matchups = season.team_matchups(args.id)?;

    // Get playoff information
    let playoffs = season.playoffs();
    let playoff_record = playoffs.record(args.id).ok();
    let playoffs_started = playoffs.started();
    let playoffs_complete = playoffs.complete();
    let is_champion = playoffs_complete && playoffs.champion() == Some(args.id);

    // Calculate total record (regular season + playoffs)
    let regular_record = matchups.record();
    let mut total_record = LeagueTeamRecord::new();
    total_record.increment_wins(*regular_record.wins());
    total_record.increment_losses(*regular_record.losses());
    total_record.increment_ties(*regular_record.ties());
    if let Some(pr) = &playoff_record {
        total_record.increment_wins(*pr.wins());
        total_record.increment_losses(*pr.losses());
        total_record.increment_ties(*pr.ties());
    }

    // Display team information for the season
    let mut tw = TabWriter::new(stdout());
    writeln!(&mut tw, "Team:\t{}", team.name()).map_err(|e| e.to_string())?;
    writeln!(&mut tw, "Record:\t{}", total_record).map_err(|e| e.to_string())?;

    // Display playoff record only if playoffs have started
    if playoffs_started {
        match &playoff_record {
            None => writeln!(&mut tw, "Playoff Record:\tN/A").map_err(|e| e.to_string())?,
            Some(pr) if *pr.wins() > 0 || *pr.losses() > 0 || *pr.ties() > 0 => {
                writeln!(&mut tw, "Playoff Record:\t{}", pr).map_err(|e| e.to_string())?;
            }
            _ => {}
        }
    }

    // Display champion status only if playoffs are complete
    if playoffs_complete && is_champion {
        writeln!(&mut tw, "Champion:\tYes").map_err(|e| e.to_string())?;
    }

    // Display playoff picture status during regular season
    if season.started() && !season.regular_season_complete() && !playoffs_started {
        if let Ok(picture) = season.playoff_picture(args.num_playoff_teams) {
            if let Some(entry) = picture.team_status(args.id) {
                let status_str = format_playoff_status(entry.status());
                writeln!(&mut tw, "Playoff Status:\t{}", status_str).map_err(|e| e.to_string())?;
                if entry.games_back() > 0.0 {
                    writeln!(&mut tw, "Games Back:\t{:.1}", entry.games_back()).map_err(|e| e.to_string())?;
                }
                if let Some(magic) = entry.magic_number() {
                    if magic > 0 {
                        writeln!(&mut tw, "Magic Number:\t{}", magic).map_err(|e| e.to_string())?;
                    }
                }
                writeln!(&mut tw, "Remaining Games:\t{}", entry.remaining_games()).map_err(|e| e.to_string())?;
            }
        }
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
                    &mut tw, "{}\tBYE\t-\tBYE\t-", i+1
                ).map_err(|e| e.to_string())?;
            },
        }
    }

    // Display playoff matchups if the team participated
    let mut has_playoff_matchups = false;
    for rounds in playoffs.conference_brackets().values() {
        for round in rounds.iter() {
            for matchup in round.matchups().iter() {
                if *matchup.home_team() == args.id || *matchup.away_team() == args.id {
                    has_playoff_matchups = true;
                    break;
                }
            }
            if has_playoff_matchups { break; }
        }
        if has_playoff_matchups { break; }
    }

    // Check winners bracket
    if !has_playoff_matchups {
        for round in playoffs.winners_bracket().iter() {
            for matchup in round.matchups().iter() {
                if *matchup.home_team() == args.id || *matchup.away_team() == args.id {
                    has_playoff_matchups = true;
                    break;
                }
            }
            if has_playoff_matchups { break; }
        }
    }
    if has_playoff_matchups {
        // Playoffs header
        writeln!(&mut tw, "\nPlayoffs").map_err(|e| e.to_string())?;
        writeln!(
            &mut tw,
            "Round\tHome Team\tHome Score\tAway Team\tAway Score"
        ).map_err(|e| e.to_string())?;

        // Display conference playoff matchups
        for (conf_index, rounds) in playoffs.conference_brackets() {
            let conf_label = season.conferences().get(*conf_index)
                .map(|c| c.name().to_string())
                .unwrap_or_else(|| format!("Conference {}", conf_index));
            for (round_index, round) in rounds.iter().enumerate() {
                for matchup in round.matchups().iter() {
                    if *matchup.home_team() == args.id || *matchup.away_team() == args.id {
                        let context = matchup.context();
                        let away_team = season.team(*matchup.away_team()).unwrap().name();
                        let home_team = season.team(*matchup.home_team()).unwrap().name();
                        writeln!(
                            &mut tw, "{} Round {}\t{}\t{}\t{}\t{}",
                            conf_label, round_index,
                            home_team, context.home_score(),
                            away_team, context.away_score()
                        ).map_err(|e| e.to_string())?;
                    }
                }
            }
        }

        // Display winners bracket matchups
        for (round_index, round) in playoffs.winners_bracket().iter().enumerate() {
            for matchup in round.matchups().iter() {
                if *matchup.home_team() == args.id || *matchup.away_team() == args.id {
                    let context = matchup.context();
                    let away_team = season.team(*matchup.away_team()).unwrap().name();
                    let home_team = season.team(*matchup.home_team()).unwrap().name();
                    writeln!(
                        &mut tw, "Championship Round {}\t{}\t{}\t{}\t{}",
                        round_index,
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

/// Format PlayoffStatus enum for display
fn format_playoff_status(status: &PlayoffStatus) -> String {
    match status {
        PlayoffStatus::ClinchedTopSeed => "Clinched #1 Seed".to_string(),
        PlayoffStatus::ClinchedPlayoffs { current_seed } => format!("Clinched (Seed #{})", current_seed),
        PlayoffStatus::InPlayoffPosition { current_seed } => format!("In Position (Seed #{})", current_seed),
        PlayoffStatus::InTheHunt => "In The Hunt".to_string(),
        PlayoffStatus::Eliminated => "Eliminated".to_string(),
    }
}
