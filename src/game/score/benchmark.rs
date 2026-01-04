use std::collections::BTreeMap;
use std::io::{Write, stdout};

use crate::cli::game::score::FbsimGameScoreBenchmarkArgs;

use fbsim_core::game::score::FinalScoreSimulator;
use fbsim_core::team::FootballTeam;

use indicatif::ProgressBar;
use tabwriter::TabWriter;
use statrs::statistics::Statistics;

pub fn final_score_sim_benchmark(_args: FbsimGameScoreBenchmarkArgs) -> Result<(), String> {
    // Instantiate the simulator and RNG
    let final_score_sim = FinalScoreSimulator::new();
    let mut rng = rand::thread_rng();

    // Instantiate 2D arrays to store the win and tie propotions
    let mut win_props = [[0.0_f64; 11]; 11];
    let mut tie_props = [[0.0_f64; 11]; 11];

    // Instantiate zeroed BTreeMap to store the score frequencies
    let mut score_freq: BTreeMap<u32, u32> = BTreeMap::new();
    for i in 0..100 {
        score_freq.insert(i as u32, 0_u32);
    }

    // Instantiate zeroed BTreeMaps to store the home and away scores
    let mut home_scores: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
    let mut away_scores: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
    for i in 0_i32..11_i32 {
        let off = (10 - i) * 10;
        let def = i * 10;
        home_scores.insert(off - def, Vec::new());
        away_scores.insert(off - def, Vec::new());
    }

    // Instantiate a progress bar for benchmark progress
    let progress_bar = ProgressBar::new(11 * 11 * 10000);

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
                "AWAY",
                home_off,
                home_def
            ).unwrap();
            let away_team = FootballTeam::from_overalls(
                "Away Team",
                "AWAY",
                away_off,
                away_def
            ).unwrap();
            for k in 1..10001 {
                // Simulate the game
                let score = final_score_sim.sim(
                    &home_team,
                    &away_team,
                    &mut rng
                ).unwrap();

                // Track the observed final score in the score frequency map
                let home_score = score.home_score();
                let away_score = score.away_score();
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
