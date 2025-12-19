use std::collections::BTreeMap;
use std::io::{Write, stdout};

use crate::cli::game::play::FbsimGamePlayBenchmarkArgs;

use fbsim_core::game::context::GameContext;
use fbsim_core::game::play::PlaySimulator;
use fbsim_core::game::play::result::{PlayResult, PlayTypeResult};
use fbsim_core::team::FootballTeam;

use indicatif::ProgressBar;
use tabwriter::TabWriter;
use statrs::statistics::Statistics;

pub fn play_benchmark(_args: FbsimGamePlayBenchmarkArgs) {
    // Instantiate the simulator and RNG
    let play_sim = PlaySimulator::new();
    let mut rng = rand::thread_rng();

    // Initialize grouped skill differential stat maps
    let mut rushes: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
    let mut passes: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
    let mut pass_distances: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
    let mut yac: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
    let mut completions: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
    let mut interceptions: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
    let mut fg_blocks: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
    let mut fg_blocks_yl: BTreeMap<u32, Vec<f64>> = BTreeMap::new();
    let mut fg_made: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
    let mut fg_made_yl: BTreeMap<u32, Vec<f64>> = BTreeMap::new();
    let mut punt_distance: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
    let mut punt_distance_yl: BTreeMap<u32, Vec<f64>> = BTreeMap::new();
    let mut punt_return_yards: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
    let mut punt_return_yards_yl: BTreeMap<u32, Vec<f64>> = BTreeMap::new();
    let mut kickoff_distance: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
    let mut kick_return_yards: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
    for i in 0..11 {
        let off = ((10 - i) * 10) as i32;
        let def = (i * 10) as i32;
        rushes.insert(off - def, Vec::new());
        passes.insert(off - def, Vec::new());
        pass_distances.insert(off - def, Vec::new());
        yac.insert(off - def, Vec::new());
        completions.insert(off - def, Vec::new());
        interceptions.insert(off - def, Vec::new());
        fg_blocks.insert(off - def, Vec::new());
        fg_made.insert(off - def, Vec::new());
        punt_distance.insert(off - def, Vec::new());
        punt_return_yards.insert(off - def, Vec::new());
        kickoff_distance.insert(off - def, Vec::new());
        kick_return_yards.insert(off - def, Vec::new());
        fg_blocks_yl.insert(i * 10, Vec::new());
        fg_made_yl.insert(i * 10, Vec::new());
        punt_distance_yl.insert(i * 10, Vec::new());
        punt_return_yards_yl.insert(i * 10, Vec::new());
    }

    // Instantiate a progress bar for benchmark progress
    let progress_bar = ProgressBar::new(11 * 11 * 100);

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

            // Calculate the skill diff
            let skill_diff = away_off as i32 - home_def as i32;

            // Create the home and away teams
            let home_team = FootballTeam::from_overalls(
                "Home Team",
                home_off,
                home_def
            ).unwrap();
            let away_team = FootballTeam::from_overalls(
                "Away Team",
                away_off,
                away_def
            ).unwrap();
            for _ in 1..101 {
                // Simulate the game
                let mut context: GameContext = GameContext::new();
                let mut game_over: bool = false;
                while !game_over {
                    game_over = *context.game_over();
                    if !game_over {
                        let (play, new_context) = play_sim.sim(&home_team, &away_team, context.clone(), &mut rng);
                        let play_result = play.result();
                        
                        // Update statistic vecs after each play
                        match play_result {
                            PlayTypeResult::Run(_) => {
                                let diff_rushes: &mut Vec<f64> = rushes.get_mut(&skill_diff).unwrap();
                                diff_rushes.push(play_result.net_yards() as f64);
                            },
                            PlayTypeResult::Pass(res) => {
                                let diff_passes: &mut Vec<f64> = passes.get_mut(&skill_diff).unwrap();
                                diff_passes.push(play_result.net_yards() as f64);
                                let diff_distances: &mut Vec<f64> = pass_distances.get_mut(&skill_diff).unwrap();
                                diff_distances.push(res.pass_dist() as f64);
                                let diff_completions: &mut Vec<f64> = completions.get_mut(&skill_diff).unwrap();
                                let diff_interceptions: &mut Vec<f64> = interceptions.get_mut(&skill_diff).unwrap();
                                if res.complete() {
                                    let diff_yac: &mut Vec<f64> = yac.get_mut(&skill_diff).unwrap();
                                    diff_yac.push(res.yards_after_catch() as f64);
                                    diff_completions.push(1.0);
                                } else {
                                    if !(res.sack() || res.scramble()) {
                                        diff_completions.push(0.0);
                                    }
                                }
                                if res.interception() {
                                    diff_interceptions.push(1.0);
                                } else {
                                    if !(res.sack() || res.scramble()) {
                                        diff_interceptions.push(0.0);
                                    }
                                }
                            },
                            PlayTypeResult::FieldGoal(res) => {
                                let yl = play.context().yards_to_touchdown();
                                let yl_group = ((yl / 10) * 10) as u32;
                                let diff_fg_blocks: &mut Vec<f64> = fg_blocks.get_mut(&skill_diff).unwrap();
                                let diff_fg_made: &mut Vec<f64> = fg_made.get_mut(&skill_diff).unwrap();
                                let yl_fg_blocks: &mut Vec<f64> = fg_blocks_yl.get_mut(&yl_group).unwrap();
                                let yl_fg_made: &mut Vec<f64> = fg_made_yl.get_mut(&yl_group).unwrap();
                                if res.made() {
                                    yl_fg_made.push(1.0);
                                    diff_fg_made.push(1.0);
                                } else {
                                    yl_fg_made.push(0.0);
                                    diff_fg_made.push(0.0);
                                }
                                if res.blocked() {
                                    yl_fg_blocks.push(1.0);
                                    diff_fg_blocks.push(1.0);
                                } else {
                                    yl_fg_blocks.push(0.0);
                                    diff_fg_blocks.push(0.0);
                                }
                            },
                            PlayTypeResult::Punt(res) => {
                                let yl = play.context().yards_to_touchdown();
                                let yl_group = ((yl / 10) * 10) as u32;
                                let diff_punt_dist: &mut Vec<f64> = punt_distance.get_mut(&skill_diff).unwrap();
                                let diff_punt_ret: &mut Vec<f64> = punt_return_yards.get_mut(&skill_diff).unwrap();
                                let yl_punt_dist: &mut Vec<f64> = punt_distance_yl.get_mut(&yl_group).unwrap();
                                let yl_punt_ret: &mut Vec<f64> = punt_return_yards_yl.get_mut(&yl_group).unwrap();
                                if !res.blocked() {
                                    diff_punt_dist.push(res.punt_yards() as f64);
                                    yl_punt_dist.push(res.punt_yards() as f64);
                                }
                                if !(res.blocked() || res.touchback() || res.out_of_bounds()) {
                                    diff_punt_ret.push(res.punt_return_yards() as f64);
                                    yl_punt_ret.push(res.punt_return_yards() as f64);
                                }
                            },
                            PlayTypeResult::Kickoff(res) => {
                                let diff_kick_dist: &mut Vec<f64> = kickoff_distance.get_mut(&skill_diff).unwrap();
                                let diff_kick_ret: &mut Vec<f64> = kick_return_yards.get_mut(&skill_diff).unwrap();
                                diff_kick_dist.push(res.kickoff_yards() as f64);
                                if !(res.touchback() || res.out_of_bounds() || res.fair_catch()) {
                                    diff_kick_ret.push(res.kick_return_yards() as f64);
                                }
                            },
                            _ => {}
                        }
                        context = new_context;
                    }
                }

                // Increment the progress bar
                progress_bar.inc(1);
            }
        }
    }

    // Rushing
    println!("");
    println!("###########");
    println!("# Rushing #");
    println!("###########");

    // Display mean, standard deviation rushing
    let mut tw = TabWriter::new(stdout());
    let mut rushing_lines = String::from("Skill Diff\tMean Rushing\tStd Rushing");
    for (diff, rs) in rushes.into_iter() {
        let mean = rs.clone().mean();
        let std = rs.clone().std_dev();
        let rushing_line = format!("{}\t{:.4}\t{:.4}", diff, mean, std);
        rushing_lines = rushing_lines + "\n" + &rushing_line;
    }
    println!("");
    println!("Rushing distribution:");
    write!(&mut tw, "{}", &rushing_lines).unwrap();
    tw.flush().unwrap();
    println!("");

    // Field goals
    println!("");
    println!("###############");
    println!("# Field goals #");
    println!("###############");

    // Display field goal made percentage by skill diff
    let mut tw = TabWriter::new(stdout());
    let mut fg_made_lines = String::from("Skill Diff\tField Goal Made Percent");
    for (diff, fgm) in fg_made.into_iter() {
        let mean = fgm.clone().mean();
        let fg_made_line = format!("{}\t{:.4}", diff, mean);
        fg_made_lines = fg_made_lines + "\n" + &fg_made_line;
    }
    println!("");
    println!("Field goal made percentages (skill):");
    write!(&mut tw, "{}", &fg_made_lines).unwrap();
    tw.flush().unwrap();
    println!("");

    // Display field goal made percentage by yard line
    let mut tw = TabWriter::new(stdout());
    let mut fg_made_yl_lines = String::from("Yard Line\tField Goal Made Percent");
    for (yl, fgm) in fg_made_yl.into_iter() {
        let mean = fgm.clone().mean();
        let fg_made_yl_line = format!("{}\t{:.4}", yl, mean);
        fg_made_yl_lines = fg_made_yl_lines + "\n" + &fg_made_yl_line;
    }
    println!("");
    println!("Field goal made percentages (yard line):");
    write!(&mut tw, "{}", &fg_made_yl_lines).unwrap();
    tw.flush().unwrap();
    println!("");

    // Display field goal blocked percentage
    let mut tw = TabWriter::new(stdout());
    let mut fg_block_lines = String::from("Skill Diff\tField Goal Block Percent");
    for (diff, fgb) in fg_blocks.into_iter() {
        let mean = fgb.clone().mean();
        let fg_block_line = format!("{}\t{:.4}", diff, mean);
        fg_block_lines = fg_block_lines + "\n" + &fg_block_line;
    }
    println!("");
    println!("Field goal block percentages:");
    write!(&mut tw, "{}", &fg_block_lines).unwrap();
    tw.flush().unwrap();
    println!("");

    // Display field goal blocked percentage by yard line
    let mut tw = TabWriter::new(stdout());
    let mut fg_blocked_yl_lines = String::from("Yard Line\tField Goal Made Percent");
    for (yl, fgb) in fg_blocks_yl.into_iter() {
        let mean = fgb.clone().mean();
        let fg_blocked_yl_line = format!("{}\t{:.4}", yl, mean);
        fg_blocked_yl_lines = fg_blocked_yl_lines + "\n" + &fg_blocked_yl_line;
    }
    println!("");
    println!("Field goal blocked percentages (yard line):");
    write!(&mut tw, "{}", &fg_blocked_yl_lines).unwrap();
    tw.flush().unwrap();
    println!("");

    // Kickoffs
    println!("");
    println!("############");
    println!("# Kickoffs #");
    println!("############");

    // Display kickoff distance by skill diff
    let mut tw = TabWriter::new(stdout());
    let mut kick_dist_lines = String::from("Skill Diff\tMean Kickoff Distance\tStd Kickoff Distance");
    for (diff, dist) in kickoff_distance.into_iter() {
        let mean = dist.clone().mean();
        let std = dist.clone().std_dev();
        let kick_dist_line = format!("{}\t{:.4}\t{:.4}", diff, mean, std);
        kick_dist_lines = kick_dist_lines + "\n" + &kick_dist_line;
    }
    println!("");
    println!("Kickoff distance distribution (skill):");
    write!(&mut tw, "{}", &kick_dist_lines).unwrap();
    tw.flush().unwrap();
    println!("");

    // Display kickoff return yards by skill diff
    let mut tw = TabWriter::new(stdout());
    let mut kick_ret_lines = String::from("Skill Diff\tMean Kick Return Yards\tStd Kick Return Yards");
    for (diff, ret) in kick_return_yards.into_iter() {
        let mean = ret.clone().mean();
        let std = ret.clone().std_dev();
        let kick_ret_line = format!("{}\t{:.4}\t{:.4}", diff, mean, std);
        kick_ret_lines = kick_ret_lines + "\n" + &kick_ret_line;
    }
    println!("");
    println!("Kick return yards distribution (skill):");
    write!(&mut tw, "{}", &kick_ret_lines).unwrap();
    tw.flush().unwrap();
    println!("");

    // Punts
    println!("");
    println!("#########");
    println!("# Punts #");
    println!("#########");

    // Display punt distance by skill diff
    let mut tw = TabWriter::new(stdout());
    let mut punt_dist_lines = String::from("Skill Diff\tMean Punt Distance\tStd Punt Distance");
    for (diff, dist) in punt_distance.into_iter() {
        let mean = dist.clone().mean();
        let std = dist.clone().std_dev();
        let punt_dist_line = format!("{}\t{:.4}\t{:.4}", diff, mean, std);
        punt_dist_lines = punt_dist_lines + "\n" + &punt_dist_line;
    }
    println!("");
    println!("Punt distance distribution (skill):");
    write!(&mut tw, "{}", &punt_dist_lines).unwrap();
    tw.flush().unwrap();
    println!("");

    // Display punt distance by yard line
    let mut tw = TabWriter::new(stdout());
    let mut punt_dist_yl_lines = String::from("Yard Line\tMean Punt Distance\tStd Punt Distance");
    for (yl, dist) in punt_distance_yl.into_iter() {
        let mean = dist.clone().mean();
        let std = dist.clone().std_dev();
        let punt_dist_yl_line = format!("{}\t{:.4}\t{:.4}", yl, mean, std);
        punt_dist_yl_lines = punt_dist_yl_lines + "\n" + &punt_dist_yl_line;
    }
    println!("");
    println!("Punt distance distribution (yard line):");
    write!(&mut tw, "{}", &punt_dist_yl_lines).unwrap();
    tw.flush().unwrap();
    println!("");

    // Display punt return yards by skill diff
    let mut tw = TabWriter::new(stdout());
    let mut punt_ret_lines = String::from("Skill Diff\tMean Punt Return Yards\tStd Punt Return Yards");
    for (diff, ret) in punt_return_yards.into_iter() {
        let mean = ret.clone().mean();
        let std = ret.clone().std_dev();
        let punt_ret_line = format!("{}\t{:.4}\t{:.4}", diff, mean, std);
        punt_ret_lines = punt_ret_lines + "\n" + &punt_ret_line;
    }
    println!("");
    println!("Punt return yards distribution (skill):");
    write!(&mut tw, "{}", &punt_ret_lines).unwrap();
    tw.flush().unwrap();
    println!("");

    // Display punt return yards by yard line
    let mut tw = TabWriter::new(stdout());
    let mut punt_ret_yl_lines = String::from("Yard Line\tMean Punt Return Yards\tStd Punt Return Yards");
    for (yl, ret) in punt_return_yards_yl.into_iter() {
        let mean = ret.clone().mean();
        let std = ret.clone().std_dev();
        let punt_ret_yl_line = format!("{}\t{:.4}\t{:.4}", yl, mean, std);
        punt_ret_yl_lines = punt_ret_yl_lines + "\n" + &punt_ret_yl_line;
    }
    println!("");
    println!("Punt return yards distribution (yard line):");
    write!(&mut tw, "{}", &punt_ret_yl_lines).unwrap();
    tw.flush().unwrap();
    println!("");

    // Passing
    println!("");
    println!("###########");
    println!("# Passing #");
    println!("###########");

    // Display mean, standard deviation passing
    let mut tw = TabWriter::new(stdout());
    let mut passing_lines = String::from("Skill Diff\tMean Passing\tStd Passing");
    for (diff, ps) in passes.into_iter() {
        let mean = ps.clone().mean();
        let std = ps.clone().std_dev();
        let passing_line = format!("{}\t{:.4}\t{:.4}", diff, mean, std);
        passing_lines = passing_lines + "\n" + &passing_line;
    }
    println!("");
    println!("Passing distribution:");
    write!(&mut tw, "{}", &passing_lines).unwrap();
    tw.flush().unwrap();
    println!("");

    // Display mean, standard deviation pass distances
    let mut tw = TabWriter::new(stdout());
    let mut pass_distance_lines = String::from("Skill Diff\tMean Pass Distance\tStd Pass Distance");
    for (diff, dists) in pass_distances.into_iter() {
        let mean = dists.clone().mean();
        let std = dists.clone().std_dev();
        let pass_distance_line = format!("{}\t{:.4}\t{:.4}", diff, mean, std);
        pass_distance_lines = pass_distance_lines + "\n" + &pass_distance_line;
    }
    println!("");
    println!("Pass distance distribution:");
    write!(&mut tw, "{}", &pass_distance_lines).unwrap();
    tw.flush().unwrap();
    println!("");

    // Display mean, standard deviation yards after catch
    let mut tw = TabWriter::new(stdout());
    let mut yac_lines = String::from("Skill Diff\tMean Yards After Catch\tStd Yards After Catch");
    for (diff, yacs) in yac.into_iter() {
        let mean = yacs.clone().mean();
        let std = yacs.clone().std_dev();
        let yac_line = format!("{}\t{:.4}\t{:.4}", diff, mean, std);
        yac_lines = yac_lines + "\n" + &yac_line;
    }
    println!("");
    println!("Yards after catch distribution:");
    write!(&mut tw, "{}", &yac_lines).unwrap();
    tw.flush().unwrap();
    println!("");

    // Display completion percentages
    let mut tw = TabWriter::new(stdout());
    let mut comp_lines = String::from("Skill Diff\tCompletion Percentage");
    for (diff, comps) in completions.into_iter() {
        let mean = comps.clone().mean();
        let comp_line = format!("{}\t{:.4}", diff, mean);
        comp_lines = comp_lines + "\n" + &comp_line;
    }
    println!("");
    println!("Completion percentages:");
    write!(&mut tw, "{}", &comp_lines).unwrap();
    tw.flush().unwrap();
    println!("");

    // Display interception percentages
    let mut tw = TabWriter::new(stdout());
    let mut int_lines = String::from("Skill Diff\tInterception Percentage");
    for (diff, ints) in interceptions.into_iter() {
        let mean = ints.clone().mean();
        let int_line = format!("{}\t{:.4}", diff, mean);
        int_lines = int_lines + "\n" + &int_line;
    }
    println!("");
    println!("Interception percentages:");
    write!(&mut tw, "{}", &int_lines).unwrap();
    tw.flush().unwrap();
    println!("");
}
