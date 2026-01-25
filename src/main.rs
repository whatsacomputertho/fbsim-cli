mod cli;
mod game;
mod league;

use std::process;

use crate::cli::fbsim::{
    FbsimCli,
    FbsimSubcommand
};
use crate::cli::game::FbsimGameSubcommand;
use crate::cli::game::drive::FbsimGameDriveSubcommand;
use crate::cli::game::play::FbsimGamePlaySubcommand;
use crate::cli::game::score::FbsimGameScoreSubcommand;
use crate::cli::league::FbsimLeagueSubcommand;
use crate::cli::league::team::FbsimLeagueTeamSubcommand;
use crate::cli::league::team::stats::FbsimLeagueTeamStatsSubcommand;
use crate::cli::league::season::FbsimLeagueSeasonSubcommand;
use crate::cli::league::season::schedule::FbsimLeagueSeasonScheduleSubcommand;
use crate::cli::league::season::team::FbsimLeagueSeasonTeamSubcommand;
use crate::cli::league::season::team::stats::FbsimLeagueSeasonTeamStatsSubcommand;
use crate::cli::league::season::playoffs::FbsimLeagueSeasonPlayoffsSubcommand;
use crate::cli::league::season::playoffs::round::FbsimLeagueSeasonPlayoffsRoundSubcommand;
use crate::cli::league::season::playoffs::round::matchup::FbsimLeagueSeasonPlayoffsRoundMatchupSubcommand;
use crate::cli::league::season::week::FbsimLeagueSeasonWeekSubcommand;
use crate::cli::league::season::week::matchup::FbsimLeagueSeasonWeekMatchupSubcommand;
use crate::cli::league::season::week::matchup::play::FbsimLeagueSeasonWeekMatchupPlaySubcommand;

use crate::game::game_sim;
use crate::game::game_benchmark;
use crate::game::drive::sim::drive_sim;
use crate::game::play::benchmark::play_benchmark;
use crate::game::play::sim::play_sim;
use crate::game::score::benchmark::final_score_sim_benchmark;
use crate::game::score::sim::final_score_sim;
use crate::league::create::create_league;
use crate::league::team::add::add_team;
use crate::league::team::get::get_team;
use crate::league::team::list::list_teams;
use crate::league::team::stats::passing::list_passing;
use crate::league::team::stats::rushing::list_rushing;
use crate::league::team::stats::receiving::list_receiving;
use crate::league::season::add::add_season;
use crate::league::season::get::get_season;
use crate::league::season::list::list_seasons;
use crate::league::season::sim::sim_season;
use crate::league::season::schedule::generate_schedule;
use crate::league::season::team::add::add_season_team;
use crate::league::season::team::get::get_season_team;
use crate::league::season::team::list::list_season_teams;
use crate::league::season::team::stats::passing::list_season_passing;
use crate::league::season::team::stats::rushing::list_season_rushing;
use crate::league::season::team::stats::receiving::list_season_receiving;
use crate::league::season::playoffs::gen::gen_playoffs;
use crate::league::season::playoffs::get::get_playoffs;
use crate::league::season::playoffs::sim::sim_playoffs;
use crate::league::season::playoffs::round::get::get_playoffs_round;
use crate::league::season::playoffs::round::sim::sim_playoffs_round;
use crate::league::season::playoffs::round::matchup::get::get_playoffs_matchup;
use crate::league::season::playoffs::round::matchup::sim::sim_playoffs_matchup;
use crate::league::season::week::get::get_season_week;
use crate::league::season::week::list::list_season_weeks;
use crate::league::season::week::sim::sim_season_week;
use crate::league::season::week::matchup::get::get_matchup;
use crate::league::season::week::matchup::sim::sim_matchup;
use crate::league::season::week::matchup::play::sim::sim_play;

use clap::Parser;

fn main() {
    // Parse the command-line args
    let fbdb_cli = FbsimCli::parse();

    // Perform the subcommand
    let command = fbdb_cli.command();
    let command_res = match &command {
        FbsimSubcommand::Game { command } => match command {
            FbsimGameSubcommand::Benchmark(args) => game_benchmark(args.clone()),
            FbsimGameSubcommand::Sim(args) => game_sim(args.clone()),
            FbsimGameSubcommand::Drive { command } => match command {
                FbsimGameDriveSubcommand::Sim(args) => drive_sim(args.clone())
            },
            FbsimGameSubcommand::Play { command } => match command {
                FbsimGamePlaySubcommand::Sim(args) => play_sim(args.clone()),
                FbsimGamePlaySubcommand::Benchmark(args) => play_benchmark(args.clone()),
            },
            FbsimGameSubcommand::Score { command } => match command {
                FbsimGameScoreSubcommand::Sim(args) => final_score_sim(args.clone()),
                FbsimGameScoreSubcommand::Benchmark(args) => final_score_sim_benchmark(args.clone())
            }
        },
        FbsimSubcommand::League { command } => match command {
            FbsimLeagueSubcommand::Create(args) => create_league(args.clone()),
            FbsimLeagueSubcommand::Team { command } => match command {
                FbsimLeagueTeamSubcommand::Add(args) => add_team(args.clone()),
                FbsimLeagueTeamSubcommand::Get(args) => get_team(args.clone()),
                FbsimLeagueTeamSubcommand::List(args) => list_teams(args.clone()),
                FbsimLeagueTeamSubcommand::Stats{ command } => match command {
                    FbsimLeagueTeamStatsSubcommand::Passing(args) => list_passing(args.clone()),
                    FbsimLeagueTeamStatsSubcommand::Receiving(args) => list_receiving(args.clone()),
                    FbsimLeagueTeamStatsSubcommand::Rushing(args) => list_rushing(args.clone())
                }
            },
            FbsimLeagueSubcommand::Season { command } => match command {
                FbsimLeagueSeasonSubcommand::Add(args) => add_season(args.clone()),
                FbsimLeagueSeasonSubcommand::Get(args) => get_season(args.clone()),
                FbsimLeagueSeasonSubcommand::List(args) => list_seasons(args.clone()),
                FbsimLeagueSeasonSubcommand::Sim(args) => sim_season(args.clone()),
                FbsimLeagueSeasonSubcommand::Team{ command } => match command {
                    FbsimLeagueSeasonTeamSubcommand::Add(args) => add_season_team(args.clone()),
                    FbsimLeagueSeasonTeamSubcommand::Get(args) => get_season_team(args.clone()),
                    FbsimLeagueSeasonTeamSubcommand::List(args) => list_season_teams(args.clone()),
                    FbsimLeagueSeasonTeamSubcommand::Stats{ command } => match command {
                        FbsimLeagueSeasonTeamStatsSubcommand::Passing(args) => list_season_passing(args.clone()),
                        FbsimLeagueSeasonTeamStatsSubcommand::Receiving(args) => list_season_receiving(args.clone()),
                        FbsimLeagueSeasonTeamStatsSubcommand::Rushing(args) => list_season_rushing(args.clone())
                    }
                },
                FbsimLeagueSeasonSubcommand::Playoffs{ command } => match command {
                    FbsimLeagueSeasonPlayoffsSubcommand::Gen(args) => gen_playoffs(args.clone()),
                    FbsimLeagueSeasonPlayoffsSubcommand::Get(args) => get_playoffs(args.clone()),
                    FbsimLeagueSeasonPlayoffsSubcommand::Sim(args) => sim_playoffs(args.clone()),
                    FbsimLeagueSeasonPlayoffsSubcommand::Round{ command } => match command {
                        FbsimLeagueSeasonPlayoffsRoundSubcommand::Get(args) => get_playoffs_round(args.clone()),
                        FbsimLeagueSeasonPlayoffsRoundSubcommand::Sim(args) => sim_playoffs_round(args.clone()),
                        FbsimLeagueSeasonPlayoffsRoundSubcommand::Matchup{ command } => match command {
                            FbsimLeagueSeasonPlayoffsRoundMatchupSubcommand::Get(args) => get_playoffs_matchup(args.clone()),
                            FbsimLeagueSeasonPlayoffsRoundMatchupSubcommand::Sim(args) => sim_playoffs_matchup(args.clone())
                        }
                    }
                },
                FbsimLeagueSeasonSubcommand::Schedule{ command } => match command {
                    FbsimLeagueSeasonScheduleSubcommand::Gen(args) => generate_schedule(args.clone())
                },
                FbsimLeagueSeasonSubcommand::Week{ command } => match command {
                    FbsimLeagueSeasonWeekSubcommand::Get(args) => get_season_week(args.clone()),
                    FbsimLeagueSeasonWeekSubcommand::List(args) => list_season_weeks(args.clone()),
                    FbsimLeagueSeasonWeekSubcommand::Sim(args) => sim_season_week(args.clone()),
                    FbsimLeagueSeasonWeekSubcommand::Matchup{ command } => match command {
                        FbsimLeagueSeasonWeekMatchupSubcommand::Get(args) => get_matchup(args.clone()),
                        FbsimLeagueSeasonWeekMatchupSubcommand::Sim(args) => sim_matchup(args.clone()),
                        FbsimLeagueSeasonWeekMatchupSubcommand::Play{ command } => match command {
                            FbsimLeagueSeasonWeekMatchupPlaySubcommand::Sim(args) => sim_play(args.clone())
                        }
                    }
                }
            }
        }
    };
    match command_res {
        Ok(()) => (),
        Err(error) => {
            println!("{}", error);
            process::exit(1);
        }
    }
}
