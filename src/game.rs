pub mod play;
pub mod drive;
pub mod score;

use std::fs;
use std::collections::BTreeMap;
use std::io::{stdout, Write};
use std::{thread, time};

use crossterm::{terminal, cursor, QueueableCommand};
use indicatif::ProgressBar;
use statrs::statistics::Statistics;
use tabwriter::TabWriter;
use rand::Rng;

use fbsim_core::game::play::{Game, GameSimulator};
use fbsim_core::game::play::result::{PlayResult, PlayTypeResult};
use fbsim_core::game::context::{GameContext, GameContextBuilder};
use fbsim_core::team::FootballTeam;

use crate::cli::game::FbsimGameBenchmarkArgs;
use crate::cli::game::FbsimGameSimArgs;

pub fn game_sim(args: FbsimGameSimArgs) -> Result<(), String> {
    // Load the home and away teams from their files
    let home_team_file_res = &fs::read_to_string(&args.home);
    let home_team_file = match home_team_file_res {
        Ok(file) => file,
        Err(e) => return Err(format!("Error loading home team file: {}", e)),
    };
    let home_team: FootballTeam = match serde_json::from_str(home_team_file) {
        Ok(team) => team,
        Err(e) => return Err(format!("Error loading home team: {}", e)),
    };
    let away_team_file_res = &fs::read_to_string(&args.away);
    let away_team_file = match away_team_file_res {
        Ok(file) => file,
        Err(e) => return Err(format!("Error loading away team file: {}", e)),
    };
    let away_team: FootballTeam = match serde_json::from_str(away_team_file) {
        Ok(team) => team,
        Err(e) => return Err(format!("Error loading away team: {}", e)),
    };

    // Load the playback speed argument
    let playback_speed: f64 = args.playback_speed.unwrap_or(2.0);

    // Initialize a new context and RNG
    let mut rng = rand::thread_rng();
    let home_opening_kickoff: bool = rng.gen::<bool>();
    let context: GameContext = GameContextBuilder::new()
        .home_team_short(home_team.short_name())
        .away_team_short(away_team.short_name())
        .home_possession(!home_opening_kickoff)
        .home_positive_direction(!home_opening_kickoff)
        .home_opening_kickoff(home_opening_kickoff)
        .build()
        .unwrap();

    // Simulate until the game is over
    let mut stdout = stdout();
    let game_sim = GameSimulator::new();
    let mut game = Game::new();
    let mut new_context = context.clone();
    while !new_context.game_over() {
        // Simulate a play
        let next_context = match game_sim.sim_play(&home_team, &away_team, new_context, &mut game, &mut rng) {
            Ok(c) => c,
            Err(e) => return Err(format!("Error simulating game: {}", e))
        };
        new_context = next_context;

        // Display the updated drive
        let drive = match game.drives().last() {
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
    }

    // Print game-over message and final stats
    println!("{} Game over", new_context);
    println!();
    println!(
        "{} stats | Passing: {} | Rushing: {} | Receiving: {}",
        home_team.short_name(),
        game.passing_stats(true),
        game.rushing_stats(true),
        game.receiving_stats(true)
    );
    println!(
        "{} stats | Passing: {} | Rushing: {} | Receiving: {}",
        away_team.short_name(),
        game.passing_stats(false),
        game.rushing_stats(false),
        game.receiving_stats(false)
    );
    Ok(())
}

pub fn game_benchmark(_args: FbsimGameBenchmarkArgs) -> Result<(), String> {
    // Instantiate the simulator and RNG
    let game_sim = GameSimulator::new();
    let mut rng = rand::thread_rng();

    // Instantiate 2D arrays to store the win and tie propotions
    let mut win_props = [[0.0_f64; 11]; 11];
    let mut tie_props = [[0.0_f64; 11]; 11];

    // Instantiate zeroed BTreeMap to store the score frequencies
    let mut score_freq: BTreeMap<u32, u32> = BTreeMap::new();
    for i in 0..1000 {
        score_freq.insert(i as u32, 0_u32);
    }

    // Instantiate zeroed BTreeMaps to store the home and away scores
    let mut home_scores: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
    let mut away_scores: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
    for i in 0..11 {
        let off = (10 - i) * 10;
        let def = i * 10;
        home_scores.insert(off - def, Vec::new());
        away_scores.insert(off - def, Vec::new());
    }

    // Instantiate a progress bar for benchmark progress
    let progress_bar = ProgressBar::new(11 * 11 * 1000);

    // Run many game simulations and track the observed
    // win and tie proportions by skill differential
    for i in 0..11 {
        // Set the home offense and away defense
        let home_off = ((10 - i) * 10) as u32;
        let away_def = (i * 10) as u32;
        for j in 0..11 {
            // Set the away offense and home defense
            let away_off = ((10 - j) * 10) as u32;
            let home_def = (j * 10) as u32;

            // Create the home and away teams
            let home_team = FootballTeam::from_overalls(
                "Home Team",
                "HOME",
                home_off,
                home_def
            ).unwrap();
            let away_team = FootballTeam::from_overalls(
                "Away Team",
                "AWAY",
                away_off,
                away_def
            ).unwrap();
            for k in 1..1001 {
                // Simulate the game
                let home_opening_kickoff: bool = rng.gen::<bool>();
                let mut context: GameContext = GameContextBuilder::new()
                    .home_possession(!home_opening_kickoff)
                    .home_positive_direction(!home_opening_kickoff)
                    .home_opening_kickoff(home_opening_kickoff)
                    .build()
                    .unwrap();
                let (_game, new_context) = match game_sim.sim(&home_team, &away_team, context.clone(), &mut rng) {
                    Ok((g, c)) => (g, c),
                    Err(e) => return Err(format!("Error simulating game: {}", e))
                };
                context = new_context;

                // Track the observed final score in the score frequency map
                let home_score = context.home_score();
                let away_score = context.away_score();
                let curr_home_score_count = score_freq.get(&home_score).unwrap();
                score_freq.insert(home_score, curr_home_score_count + 1_u32);
                let curr_away_score_count = score_freq.get(&away_score).unwrap();
                score_freq.insert(away_score, curr_away_score_count + 1_u32);

                // Track the observed home and away scores in the home/away score maps
                let diff_home_scores: &mut Vec<f64> = home_scores.get_mut(&(home_off as i32 - away_def as i32)).unwrap();
                diff_home_scores.push(home_score as f64);
                let diff_away_scores: &mut Vec<f64> = away_scores.get_mut(&(away_off as i32 - home_def as i32)).unwrap();
                diff_away_scores.push(away_score as f64);

                // Decide whether this was a tie, or home win / away win
                let was_tie = home_score == away_score;
                let home_win = home_score > away_score;

                // Increment the tie proportions
                tie_props[i][j] = if was_tie {
                    ((tie_props[i][j] * ((k as f64) - 1_f64)) + 1_f64) / (k as f64)
                } else {
                    ((tie_props[i][j] * ((k as f64) - 1_f64)) + 0_f64) / (k as f64)
                };

                // Increment the win proportions
                win_props[i][j] = if home_win {
                    (win_props[i][j] * ((k as f64) - 1_f64) + 1_f64) / (k as f64)
                } else {
                    (win_props[i][j] * ((k as f64) - 1_f64) + 0_f64) / (k as f64)
                };

                // Increment the progress bar
                progress_bar.inc(1);
            }
        }
    }

    // Display the win probability table
    println!();
    let mut tw = TabWriter::new(stdout());
    let mut win_table_lines = String::from(
        "\t0\t\t\t\t\t\t\t\t\t\t100"
    );
    for (i, prop) in win_props.iter().enumerate() {
        let table_line = prop.iter().map(|x| format!("{:.4}", x)).collect::<Vec<_>>().join("\t");
        let table_line_pfx = if i == 0 || i == 10 {
            format!("{}", i * 10)
        } else {
            String::from("")
        };
        win_table_lines = win_table_lines + "\n" + &table_line_pfx + "\t" + &table_line;
    }
    println!();
    println!("Win probabilities:");
    write!(&mut tw, "{}", &win_table_lines).unwrap();
    tw.flush().unwrap();

    // Display the tie probability table
    println!();
    let mut tie_table_lines = String::from(
        "\t0\t\t\t\t\t\t\t\t\t\t100"
    );
    for (i, prop) in tie_props.iter().enumerate() {
        let table_line = prop.iter().map(|x| format!("{:.4}", x)).collect::<Vec<_>>().join("\t");
        let table_line_pfx = if i == 0 || i == 10 {
            format!("{}", i * 10)
        } else {
            String::from("")
        };
        tie_table_lines = tie_table_lines + "\n" + &table_line_pfx + "\t" + &table_line;
    }
    println!();
    println!("Tie probabilities:");
    write!(&mut tw, "{}", &tie_table_lines).unwrap();
    tw.flush().unwrap();
    println!();

    // Display the home mean and standard deviation score
    let mut home_mean_std_lines = String::from("Skill Diff\tMean Score\tStd Score");
    for (diff, scores) in home_scores.into_iter() {
        let mean = scores.clone().mean();
        let std = scores.clone().std_dev();
        let home_mean_std_line = format!("{}\t{:.4}\t{:.4}", diff, mean, std);
        home_mean_std_lines = home_mean_std_lines + "\n" + &home_mean_std_line;
    }
    println!();
    println!("Home score distribution:");
    write!(&mut tw, "{}", &home_mean_std_lines).unwrap();
    tw.flush().unwrap();
    println!();

    // Display the away mean and standard deviation score
    let mut away_mean_std_lines = String::from("Skill Diff\tMean Score\tStd Score");
    for (diff, scores) in away_scores.into_iter() {
        let mean = scores.clone().mean();
        let std = scores.clone().std_dev();
        let away_mean_std_line = format!("{}\t{:.4}\t{:.4}", diff, mean, std);
        away_mean_std_lines = away_mean_std_lines + "\n" + &away_mean_std_line;
    }
    println!();
    println!("Away score distribution:");
    write!(&mut tw, "{}", &away_mean_std_lines).unwrap();
    tw.flush().unwrap();
    println!();

    // Display the observed score frequency
    let mut score_freq_table_lines = String::from("Score\tFrequency\tCount");
    for (score, count) in score_freq.into_iter() {
        let freq = count as f64 / (11_f64 * 11_f64 * 10000_f64);
        let score_freq_table_line = format!("{}\t{:.4}%\t{}", score, freq * 100_f64, count);
        score_freq_table_lines = score_freq_table_lines + "\n" + &score_freq_table_line;
    }
    println!();
    println!("Score frequency:");
    write!(&mut tw, "{}", &score_freq_table_lines).unwrap();
    tw.flush().unwrap();
    println!();
    Ok(())
}
