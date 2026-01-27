use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;

use crate::cli::league::season::standings::FbsimLeagueSeasonStandingsArgs;

use serde_json;
use tabwriter::TabWriter;

pub fn get_standings(args: FbsimLeagueSeasonStandingsArgs) -> Result<(), String> {
    // Validate args: division requires conference
    if args.division.is_some() && args.conference.is_none() {
        return Err(String::from("Division filter requires conference filter (-c/--conference)"));
    }

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

    // Get the season
    let season = match league.season(args.year) {
        Some(season) => season,
        None => return Err(format!("No season found with year: {}", args.year)),
    };

    let conferences = season.conferences();
    let has_conferences = !conferences.is_empty();

    // Handle different display modes
    if args.by_division {
        if !has_conferences {
            return Err(String::from("No conferences/divisions defined for this season"));
        }
        display_standings_by_division(season)?;
    } else if args.by_conference {
        if !has_conferences {
            return Err(String::from("No conferences defined for this season"));
        }
        display_standings_by_conference(season)?;
    } else if let Some(conf_idx) = args.conference {
        // Filter by specific conference
        if let Some(div_id) = args.division {
            display_division_standings(season, conf_idx, div_id)?;
        } else {
            display_conference_standings(season, conf_idx)?;
        }
    } else {
        // Display overall standings
        display_overall_standings(season)?;
    }

    Ok(())
}

fn display_overall_standings(season: &fbsim_core::league::season::LeagueSeason) -> Result<(), String> {
    let standings = season.standings();

    let mut tw = TabWriter::new(stdout());
    writeln!(&mut tw, "Rank\tTeam\tRecord").map_err(|e| e.to_string())?;

    for (rank, (id, _)) in standings.iter().enumerate() {
        let team = season.team(*id).unwrap();
        let record = season.team_matchups(*id)
            .map(|m| m.record().to_string())
            .unwrap_or_else(|_| String::from("-"));
        writeln!(
            &mut tw, "{}\t{}\t{}",
            rank + 1,
            team.name(),
            record
        ).map_err(|e| e.to_string())?;
    }
    tw.flush().map_err(|e| e.to_string())?;
    Ok(())
}

fn display_standings_by_conference(
    season: &fbsim_core::league::season::LeagueSeason
) -> Result<(), String> {
    let conferences = season.conferences();

    for (conf_idx, conference) in conferences.iter().enumerate() {
        println!("=== {} ===", conference.name());

        let standings = season.conference_standings(conf_idx)?;

        let mut tw = TabWriter::new(stdout());
        writeln!(&mut tw, "Rank\tTeam\tRecord").map_err(|e| e.to_string())?;

        for (rank, (id, _)) in standings.iter().enumerate() {
            let team = season.team(*id).unwrap();
            let record = season.team_matchups(*id)
                .map(|m| m.record().to_string())
                .unwrap_or_else(|_| String::from("-"));
            writeln!(
                &mut tw, "{}\t{}\t{}",
                rank + 1,
                team.name(),
                record
            ).map_err(|e| e.to_string())?;
        }
        tw.flush().map_err(|e| e.to_string())?;
        println!();
    }
    Ok(())
}

fn display_standings_by_division(
    season: &fbsim_core::league::season::LeagueSeason
) -> Result<(), String> {
    let conferences = season.conferences();

    for (conf_idx, conference) in conferences.iter().enumerate() {
        println!("=== {} ===", conference.name());

        for (div_id, division) in conference.divisions() {
            println!("--- {} ---", division.name());

            let standings = season.division_standings(conf_idx, *div_id)?;

            let mut tw = TabWriter::new(stdout());
            writeln!(&mut tw, "Rank\tTeam\tRecord").map_err(|e| e.to_string())?;

            for (rank, (id, _)) in standings.iter().enumerate() {
                let team = season.team(*id).unwrap();
                let record = season.team_matchups(*id)
                    .map(|m| m.record().to_string())
                    .unwrap_or_else(|_| String::from("-"));
                writeln!(
                    &mut tw, "{}\t{}\t{}",
                    rank + 1,
                    team.name(),
                    record
                ).map_err(|e| e.to_string())?;
            }
            tw.flush().map_err(|e| e.to_string())?;
            println!();
        }
    }
    Ok(())
}

fn display_conference_standings(
    season: &fbsim_core::league::season::LeagueSeason,
    conf_idx: usize
) -> Result<(), String> {
    let conferences = season.conferences();
    let conference = match conferences.get(conf_idx) {
        Some(c) => c,
        None => return Err(format!("No conference found with index: {}", conf_idx)),
    };

    println!("=== {} ===", conference.name());

    let standings = season.conference_standings(conf_idx)?;

    let mut tw = TabWriter::new(stdout());
    writeln!(&mut tw, "Rank\tTeam\tRecord").map_err(|e| e.to_string())?;

    for (rank, (id, _)) in standings.iter().enumerate() {
        let team = season.team(*id).unwrap();
        let record = season.team_matchups(*id)
            .map(|m| m.record().to_string())
            .unwrap_or_else(|_| String::from("-"));
        writeln!(
            &mut tw, "{}\t{}\t{}",
            rank + 1,
            team.name(),
            record
        ).map_err(|e| e.to_string())?;
    }
    tw.flush().map_err(|e| e.to_string())?;
    Ok(())
}

fn display_division_standings(
    season: &fbsim_core::league::season::LeagueSeason,
    conf_idx: usize,
    div_id: usize
) -> Result<(), String> {
    let conferences = season.conferences();
    let conference = match conferences.get(conf_idx) {
        Some(c) => c,
        None => return Err(format!("No conference found with index: {}", conf_idx)),
    };

    let division = match conference.division(div_id) {
        Some(d) => d,
        None => return Err(format!("No division found with ID: {}", div_id)),
    };

    println!("=== {} - {} ===", conference.name(), division.name());

    let standings = season.division_standings(conf_idx, div_id)?;

    let mut tw = TabWriter::new(stdout());
    writeln!(&mut tw, "Rank\tTeam\tRecord").map_err(|e| e.to_string())?;

    for (rank, (id, _)) in standings.iter().enumerate() {
        let team = season.team(*id).unwrap();
        let record = season.team_matchups(*id)
            .map(|m| m.record().to_string())
            .unwrap_or_else(|_| String::from("-"));
        writeln!(
            &mut tw, "{}\t{}\t{}",
            rank + 1,
            team.name(),
            record
        ).map_err(|e| e.to_string())?;
    }
    tw.flush().map_err(|e| e.to_string())?;
    Ok(())
}
