use std::fs;
use std::io::{Write, stdout};

use fbsim_core::league::League;
use fbsim_core::league::season::playoffs::picture::PlayoffStatus;

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

    // Get the playoff picture
    let picture = season.playoff_picture(args.num_playoff_teams)?;

    // Display header
    println!("Playoff Picture for {} Season", args.year);
    println!("Top {} teams make the playoffs", args.num_playoff_teams);
    println!("Weeks remaining in season: {}", weeks_remaining);
    println!();

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

    // Display legend
    println!("Legend:");
    println!("  z- = Clinched #1 seed");
    println!("  x- = Clinched playoff berth");
    println!("  GB = Games behind playoff cutoff");
    println!("  Magic # = Wins needed to clinch (X = clinched)");
    Ok(())
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
