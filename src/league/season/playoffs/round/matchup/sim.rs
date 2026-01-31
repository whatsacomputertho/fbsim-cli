use std::fs;
use std::io::{stdout, Write};
use std::{thread, time};

use crossterm::{cursor, terminal, QueueableCommand};

use fbsim_core::league::League;
use fbsim_core::league::season::matchup::LeagueSeasonMatchup;
use fbsim_core::game::play::{Drive, Game};
use fbsim_core::game::play::result::{PlayResult, PlayTypeResult};

use crate::cli::league::season::playoffs::round::matchup::FbsimLeagueSeasonPlayoffsRoundMatchupSimArgs;

use serde_json;

pub fn sim_playoffs_matchup(args: FbsimLeagueSeasonPlayoffsRoundMatchupSimArgs) -> Result<(), String> {
    // Load the league from its file as mutable
    let file_res = &fs::read_to_string(&args.league);
    let file = match file_res {
        Ok(file) => file,
        Err(error) => return Err(format!("Error loading league file: {}", error)),
    };
    let league_res = serde_json::from_str(file);
    let mut league: League = match league_res {
        Ok(league) => league,
        Err(error) => return Err(format!("Error loading league from file: {}", error)),
    };

    // Load the playback speed argument
    let playback_speed: f64 = args.playback_speed.unwrap_or(2.0);

    // Simulate the matchup play-by-play
    let mut rng = rand::thread_rng();
    let mut stdout = stdout();
    loop {
        // Get the current season mutably and simulate a play
        let game_opt: Option<Game> = {
            let season = match league.current_season_mut() {
                Some(s) => s,
                None => return Err(String::from("No current season found")),
            };
            if args.winners_bracket {
                match season.sim_winners_bracket_play(args.round, args.matchup, &mut rng) {
                    Ok(game_opt) => game_opt,
                    Err(error) => return Err(format!("Error simulating next play for winners bracket matchup: {}", error)),
                }
            } else {
                match season.sim_playoff_play(args.conference, args.round, args.matchup, &mut rng) {
                    Ok(game_opt) => game_opt,
                    Err(error) => return Err(format!("Error simulating next play for playoff matchup: {}", error)),
                }
            }
        };

        // Get the season immutably to read the matchup
        let season = match league.current_season() {
            Some(s) => s,
            None => return Err(String::from("No current season found after simulating play"))
        };
        let matchup = get_matchup(season.playoffs(), &args)?;
        let drive_opt: Option<&Drive> = if let Some(g) = game_opt.as_ref() {
            g.drives().last()
        } else {
            match matchup.game() {
                Some(g) => g.drives().last(),
                None => return Err(String::from("Failed to get game after simulating play"))
            }
        };

        // Display the latest drive in the game
        let drive = match drive_opt {
            Some(d) => d,
            None => return Err(String::from("No drive found in current game"))
        };
        let drive_str = format!("{}", drive);
        let drive_str_len = drive_str.matches("\n").count() as u16;
        if stdout.write_all(drive_str.as_bytes()).is_err() {
            return Err(String::from("Failed to write drive to stdout"));
        }
        if stdout.flush().is_err() {
            return Err(String::from("Failed to flush stdout"));
        }

        // Wait based on the duration of the play
        let play = match drive.plays().last() {
            Some(p) => p,
            None => return Err(String::from("No plays found in current drive"))
        };
        let play_duration = play.result().play_duration();
        let post_play_duration = match play.post_play() {
            PlayTypeResult::BetweenPlay(res) => 20.max(res.duration()),
            _ => 30
        };
        let duration = play_duration + post_play_duration;
        let wait_time = (duration * 250) as f64 / playback_speed;
        let one_sec = time::Duration::from_millis(wait_time.round().abs() as u64);
        thread::sleep(one_sec);

        // Reset the cursor if drive is not complete
        if !drive.complete() {
            let errmsg = String::from("Failed to reset cursor");
            if stdout.queue(cursor::MoveUp(drive_str_len)).is_err() {
                return Err(errmsg);
            }
            if stdout.queue(cursor::MoveToColumn(0)).is_err() {
                return Err(errmsg);
            }
            if stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).is_err() {
                return Err(errmsg);
            }
        } else {
            println!("\n");
        }

        // Break if the game is over
        if matchup.context().game_over() {
            break
        }
    }

    // Print game-over message and final stats
    let season = match league.current_season() {
        Some(s) => s,
        None => return Err(String::from("No current season found after simulating game"))
    };
    let matchup = get_matchup(season.playoffs(), &args)?;
    let home_stats = match matchup.home_stats() {
        Some(s) => s,
        None => return Err(String::from("Failed to get home stats after simulating game"))
    };
    let away_stats = match matchup.away_stats() {
        Some(s) => s,
        None => return Err(String::from("Failed to get away stats after simulating game"))
    };
    let context = matchup.context();
    println!("{} Game over", context);
    println!();
    println!(
        "{} stats\n{}",
        context.home_team_short(),
        home_stats
    );
    println!();
    println!(
        "{} stats\n{}",
        context.away_team_short(),
        away_stats
    );

    // Try to generate the next round if playoffs are not yet complete
    if !season.playoffs().complete() {
        let season = match league.current_season_mut() {
            Some(s) => s,
            None => return Err(String::from("No current season found for generating next round")),
        };
        if season.generate_next_playoff_round(&mut rng).is_ok() {
            println!("\nNext playoff round generated!");
        }
    }

    // Serialize the league as JSON
    let league_res = serde_json::to_string_pretty(&league);
    let league_str: String = match league_res {
        Ok(league_str) => league_str,
        Err(error) => return Err(format!("Error serializing league: {}", error)),
    };

    // Write the league back to its file
    let write_res = fs::write(&args.league, league_str);
    if let Err(e) = write_res {
        return Err(format!("Error writing league file: {}", e));
    }
    Ok(())
}

fn get_matchup<'a>(
    playoffs: &'a fbsim_core::league::season::playoffs::LeagueSeasonPlayoffs,
    args: &FbsimLeagueSeasonPlayoffsRoundMatchupSimArgs
) -> Result<&'a LeagueSeasonMatchup, String> {
    let round = if args.winners_bracket {
        let winners = playoffs.winners_bracket();
        match winners.get(args.round) {
            Some(r) => r,
            None => return Err(format!("No winners bracket round found with index: {}", args.round)),
        }
    } else {
        let bracket = match playoffs.conference_bracket(args.conference) {
            Some(b) => b,
            None => return Err(format!("No conference bracket found with index: {}", args.conference)),
        };
        match bracket.get(args.round) {
            Some(r) => r,
            None => return Err(format!("No playoff round found with index: {}", args.round)),
        }
    };
    match round.matchups().get(args.matchup) {
        Some(m) => Ok(m),
        None => Err(format!("No matchup found with index: {}", args.matchup)),
    }
}
