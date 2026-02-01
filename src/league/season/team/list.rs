use std::collections::HashMap;
use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;
use fbsim_core::league::season::matchup::LeagueSeasonMatchups;
use fbsim_core::league::season::playoffs::picture::PlayoffStatus;

use crate::cli::league::season::team::FbsimLeagueSeasonTeamListArgs;

use serde_json;
use tabwriter::TabWriter;

pub fn list_season_teams(args: FbsimLeagueSeasonTeamListArgs) -> Result<(), String> {
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

    let playoffs = season.playoffs();
    let playoffs_started = playoffs.started();
    let playoffs_complete = playoffs.complete();
    let champion_id = playoffs.champion();

    // Get playoff picture if regular season is in progress (started but not complete, playoffs not started)
    let playoff_picture = if season.started() && !season.regular_season_complete() && !playoffs_started {
        season.playoff_picture(args.num_playoff_teams).ok()
    } else {
        None
    };

    // Determine if we need conference/division columns
    let conferences = season.conferences();
    let show_conference = conferences.len() > 1;
    let show_division = conferences.iter().any(|c| c.divisions().len() > 1);

    // Build team -> conference/division lookup maps
    let mut team_conference: HashMap<usize, String> = HashMap::new();
    let mut team_division: HashMap<usize, String> = HashMap::new();
    if show_conference || show_division {
        for conference in conferences.iter() {
            for division in conference.divisions().iter() {
                for team_id in division.teams().iter() {
                    if show_conference {
                        team_conference.insert(*team_id, conference.name().to_string());
                    }
                    if show_division {
                        let div_name = division.name();
                        let div_str = if div_name.is_empty() {
                            "-".to_string()
                        } else {
                            div_name.to_string()
                        };
                        team_division.insert(*team_id, div_str);
                    }
                }
            }
        }
    }

    // Get standings for proper ordering
    let standings = season.standings();

    // Display the results in a table
    let mut tw = TabWriter::new(stdout());

    // Build header
    let mut header = String::from("Team");
    if show_conference { header.push_str("\tConference"); }
    if show_division { header.push_str("\tDivision"); }
    header.push_str("\tRecord");
    if playoffs_complete {
        header.push_str("\tPlayoffs\tChampion");
    } else if playoffs_started {
        header.push_str("\tPlayoffs");
    } else if playoff_picture.is_some() {
        header.push_str("\tStatus\tGB\tMagic #");
    }
    writeln!(&mut tw, "{}", header).map_err(|e| e.to_string())?;

    for (id, _) in standings.iter() {
        let team = season.team(*id).unwrap();
        let matchups: LeagueSeasonMatchups = season.team_matchups(*id)?;

        // Build conference/division prefix
        let mut prefix = team.name().to_string();
        if show_conference {
            let conf = team_conference.get(id).map(|s| s.as_str()).unwrap_or("-");
            prefix.push_str(&format!("\t{}", conf));
        }
        if show_division {
            let div = team_division.get(id).map(|s| s.as_str()).unwrap_or("-");
            prefix.push_str(&format!("\t{}", div));
        }

        if playoffs_complete {
            let playoff_record_str = match playoffs.record(*id) {
                Ok(playoff_record) => playoff_record.to_string(),
                Err(_) => String::from("-"),
            };
            let champion_str = if champion_id == Some(*id) { "X" } else { "" };
            writeln!(
                &mut tw, "{}\t{}\t{}\t{}",
                prefix, matchups.record(), playoff_record_str, champion_str
            ).map_err(|e| e.to_string())?;
        } else if playoffs_started {
            let playoff_record_str = match playoffs.record(*id) {
                Ok(playoff_record) => playoff_record.to_string(),
                Err(_) => String::from("-"),
            };
            writeln!(
                &mut tw, "{}\t{}\t{}",
                prefix, matchups.record(), playoff_record_str
            ).map_err(|e| e.to_string())?;
        } else if let Some(ref picture) = playoff_picture {
            if let Some(entry) = picture.team_status(*id) {
                let status_str = format_short_status(entry.status());
                let gb_str = if entry.games_back() > 0.0 {
                    format!("{:.1}", entry.games_back())
                } else {
                    "-".to_string()
                };
                let magic_str = match entry.magic_number() {
                    Some(0) => "X".to_string(),
                    Some(m) => m.to_string(),
                    None => "-".to_string(),
                };
                writeln!(
                    &mut tw, "{}\t{}\t{}\t{}\t{}",
                    prefix, matchups.record(), status_str, gb_str, magic_str
                ).map_err(|e| e.to_string())?;
            } else {
                writeln!(
                    &mut tw, "{}\t{}\t-\t-\t-",
                    prefix, matchups.record()
                ).map_err(|e| e.to_string())?;
            }
        } else {
            writeln!(
                &mut tw, "{}\t{}",
                prefix, matchups.record()
            ).map_err(|e| e.to_string())?;
        }
    }
    tw.flush().map_err(|e| e.to_string())?;
    Ok(())
}

/// Format PlayoffStatus enum for short table display
fn format_short_status(status: &PlayoffStatus) -> String {
    match status {
        PlayoffStatus::ClinchedTopSeed => "z-".to_string(),
        PlayoffStatus::ClinchedPlayoffs { .. } => "x-".to_string(),
        PlayoffStatus::InPlayoffPosition { current_seed } => format!("({})", current_seed),
        PlayoffStatus::InTheHunt => "*".to_string(),
        PlayoffStatus::Eliminated => "e-".to_string(),
    }
}
