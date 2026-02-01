use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;
use fbsim_core::league::season::LeagueSeason;
use fbsim_core::league::season::playoffs::picture::{PlayoffPicture, PlayoffPictureOptions, PlayoffStatus};

use crate::cli::league::season::playoffs::FbsimLeagueSeasonPlayoffsPictureArgs;

use serde_json;
use tabwriter::TabWriter;

pub fn get_playoffs_picture(args: FbsimLeagueSeasonPlayoffsPictureArgs) -> Result<(), String> {
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

    // Check if playoffs have already started
    if season.playoffs().started() {
        println!("Playoffs have already started for the {} season.", args.year);
        println!("Use 'league season playoffs get' to view the playoff bracket.");
        return Ok(());
    }

    // Check if season has a schedule
    if season.weeks().is_empty() {
        return Err(format!("Season {} has no schedule generated yet.", args.year));
    }
    let weeks_remaining = season.weeks().iter().filter(
            |w| !w.complete()
        ).count();

    // Determine if we should use conference-based playoff picture
    let has_conferences = !season.conferences().is_empty();
    if args.per_conference {
        if !has_conferences {
            return Err(String::from(
                "Per-conference playoff picture (-p) requires conferences to be defined. \
                Use 'league season conference add' first."
            ));
        }
        display_conference_playoff_picture(season, &args, weeks_remaining)?;
    } else if has_conferences && args.conference.is_some() {
        // Display single conference
        let conf_index = args.conference.unwrap();
        display_single_conference_picture(season, conf_index, &args, weeks_remaining)?;
    } else {
        // Display traditional playoff picture
        display_traditional_playoff_picture(season, &args, weeks_remaining)?;
    }
    Ok(())
}

fn display_traditional_playoff_picture(
    season: &LeagueSeason,
    args: &FbsimLeagueSeasonPlayoffsPictureArgs,
    weeks_remaining: usize
) -> Result<(), String> {
    // Get the playoff picture (explicitly non-conference)
    let options = PlayoffPictureOptions {
        by_conference: Some(false),
        division_winners_guaranteed: args.division_winners,
    };
    let picture = PlayoffPicture::from_season(season, args.num_playoff_teams, Some(options))?;

    // Display playoff picture
    println!("Playoff Picture for {} Season", args.year);
    println!("Top {} teams make the playoffs", args.num_playoff_teams);
    println!("Weeks remaining in season: {}", weeks_remaining);
    println!();
    display_playoff_picture_sections(&picture)?;
    display_legend();
    Ok(())
}

fn display_conference_playoff_picture(
    season: &LeagueSeason,
    args: &FbsimLeagueSeasonPlayoffsPictureArgs,
    weeks_remaining: usize
) -> Result<(), String> {
    let conferences = season.conferences();

    // Display header
    println!("Playoff Picture for {} Season", args.year);
    println!("{} teams per conference make the playoffs", args.num_playoff_teams);
    println!("Weeks remaining in season: {}", weeks_remaining);
    println!();

    // Get the conference-based playoff picture
    let options = PlayoffPictureOptions {
        by_conference: Some(true),
        division_winners_guaranteed: args.division_winners,
    };
    let picture = PlayoffPicture::from_season(
        season,
        args.num_playoff_teams,
        Some(options)
    )?;
    for (conf_index, conference) in conferences.iter().enumerate() {
        // Skip if filtering to specific conference
        if let Some(filter_conf) = args.conference {
            if filter_conf != conf_index {
                continue;
            }
        }

        // Display the playoff-picture
        println!("=== {} Playoff Picture ===", conference.name());
        display_conference_playoff_picture_sections(&picture, season, conf_index)?;
    }
    display_legend();
    Ok(())
}

fn display_conference_playoff_picture_sections(
    picture: &PlayoffPicture,
    season: &LeagueSeason,
    conf_index: usize
) -> Result<(), String> {
    let conference = match season.conferences().get(conf_index) {
        Some(c) => c,
        None => return Err(format!("No conference found with ID: {}", conf_index)),
    };
    let conf_teams: Vec<usize> = conference.all_teams();

    // Display teams in playoff position that are in this conference
    let binding_playoff = picture.playoff_teams();
    let playoff_teams: Vec<_> = binding_playoff.iter()
        .filter(|e| conf_teams.contains(&e.team_id()))
        .collect();
    if !playoff_teams.is_empty() {
        println!("IN PLAYOFF POSITION");
        let mut tw = TabWriter::new(stdout());
        writeln!(&mut tw, "Seed\tTeam\tRecord\tStatus\tMagic #").map_err(|e| e.to_string())?;
        for (i, entry) in playoff_teams.iter().enumerate() {
            let status_str = format_status_indicator(entry.status());
            let magic_str = match entry.magic_number() {
                Some(0) => "X".to_string(),
                Some(m) => m.to_string(),
                None => "-".to_string(),
            };
            writeln!(
                &mut tw,
                "{}\t{}\t{}\t{}\t{}",
                i + 1,
                entry.team_name(),
                entry.current_record(),
                status_str,
                magic_str
            ).map_err(|e| e.to_string())?;
        }
        tw.flush().map_err(|e| e.to_string())?;
        println!();
    }

    // Display teams in the hunt that are in this conference
    let binding_hunt = picture.in_the_hunt();
    let in_the_hunt: Vec<_> = binding_hunt.iter()
        .filter(|e| conf_teams.contains(&e.team_id()))
        .collect();
    if !in_the_hunt.is_empty() {
        println!("IN THE HUNT");
        let mut tw = TabWriter::new(stdout());
        writeln!(&mut tw, "Team\tRecord\tGB\tRemaining").map_err(|e| e.to_string())?;
        for entry in in_the_hunt.iter() {
            writeln!(
                &mut tw,
                "{}\t{}\t{:.1}\t{}",
                entry.team_name(),
                entry.current_record(),
                entry.games_back(),
                entry.remaining_games()
            ).map_err(|e| e.to_string())?;
        }
        tw.flush().map_err(|e| e.to_string())?;
        println!();
    }

    // Display eliminated teams that are in this conference
    let binding_elim = picture.eliminated_teams();
    let eliminated: Vec<_> = binding_elim.iter()
        .filter(|e| conf_teams.contains(&e.team_id()))
        .collect();
    if !eliminated.is_empty() {
        println!("ELIMINATED");
        let mut tw = TabWriter::new(stdout());
        writeln!(&mut tw, "Team\tRecord\tGB").map_err(|e| e.to_string())?;
        for entry in eliminated.iter() {
            writeln!(
                &mut tw,
                "{}\t{}\t{:.1}",
                entry.team_name(),
                entry.current_record(),
                entry.games_back()
            ).map_err(|e| e.to_string())?;
        }
        tw.flush().map_err(|e| e.to_string())?;
        println!();
    }
    Ok(())
}

fn display_single_conference_picture(
    season: &LeagueSeason,
    conf_index: usize,
    args: &FbsimLeagueSeasonPlayoffsPictureArgs,
    weeks_remaining: usize
) -> Result<(), String> {
    let conferences = season.conferences();
    let conference = match conferences.get(conf_index) {
        Some(c) => c,
        None => return Err(format!("No conference found with ID: {}", conf_index)),
    };

    // Display header
    println!("{} Playoff Picture for {} Season", conference.name(), args.year);
    println!("Top {} teams make the playoffs", args.num_playoff_teams);
    println!("Weeks remaining in season: {}", weeks_remaining);
    println!();

    // Use regular playoff picture but filtered to conference teams
    let options = PlayoffPictureOptions {
        by_conference: Some(false),
        division_winners_guaranteed: args.division_winners,
    };
    let picture = PlayoffPicture::from_season(season, args.num_playoff_teams, Some(options))?;
    display_playoff_picture_sections(&picture)?;
    display_legend();
    Ok(())
}

fn display_playoff_picture_sections(picture: &PlayoffPicture) -> Result<(), String> {
    // Display teams in playoff position
    let playoff_teams = picture.playoff_teams();
    if !playoff_teams.is_empty() {
        println!("IN PLAYOFF POSITION");
        let mut tw = TabWriter::new(stdout());
        writeln!(&mut tw, "Seed\tTeam\tRecord\tStatus\tMagic #").map_err(|e| e.to_string())?;
        for (i, entry) in playoff_teams.iter().enumerate() {
            let status_str = format_status_indicator(entry.status());
            let magic_str = match entry.magic_number() {
                Some(0) => "X".to_string(),
                Some(m) => m.to_string(),
                None => "-".to_string(),
            };
            writeln!(
                &mut tw,
                "{}\t{}\t{}\t{}\t{}",
                i + 1,
                entry.team_name(),
                entry.current_record(),
                status_str,
                magic_str
            ).map_err(|e| e.to_string())?;
        }
        tw.flush().map_err(|e| e.to_string())?;
        println!();
    }

    // Display teams in the hunt
    let in_the_hunt = picture.in_the_hunt();
    if !in_the_hunt.is_empty() {
        println!("IN THE HUNT");
        let mut tw = TabWriter::new(stdout());
        writeln!(&mut tw, "Team\tRecord\tGB\tRemaining").map_err(|e| e.to_string())?;
        for entry in in_the_hunt.iter() {
            writeln!(
                &mut tw,
                "{}\t{}\t{:.1}\t{}",
                entry.team_name(),
                entry.current_record(),
                entry.games_back(),
                entry.remaining_games()
            ).map_err(|e| e.to_string())?;
        }
        tw.flush().map_err(|e| e.to_string())?;
        println!();
    }

    // Display eliminated teams
    let eliminated = picture.eliminated_teams();
    if !eliminated.is_empty() {
        println!("ELIMINATED");
        let mut tw = TabWriter::new(stdout());
        writeln!(&mut tw, "Team\tRecord\tGB").map_err(|e| e.to_string())?;
        for entry in eliminated.iter() {
            writeln!(
                &mut tw,
                "{}\t{}\t{:.1}",
                entry.team_name(),
                entry.current_record(),
                entry.games_back()
            ).map_err(|e| e.to_string())?;
        }
        tw.flush().map_err(|e| e.to_string())?;
        println!();
    }
    Ok(())
}

fn display_legend() {
    println!("Legend:");
    println!("  z- = Clinched #1 seed");
    println!("  x- = Clinched playoff berth");
    println!("  GB = Games behind playoff cutoff");
    println!("  Magic # = Wins needed to clinch (X = clinched)");
}

fn format_status_indicator(status: &PlayoffStatus) -> String {
    match status {
        PlayoffStatus::ClinchedTopSeed => "z-".to_string(),
        PlayoffStatus::ClinchedPlayoffs { .. } => "x-".to_string(),
        PlayoffStatus::InPlayoffPosition { .. } => "-".to_string(),
        PlayoffStatus::InTheHunt => "-".to_string(),
        PlayoffStatus::Eliminated => "e-".to_string(),
    }
}
