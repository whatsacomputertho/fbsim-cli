mod cli;
mod game;
mod league;

use std::process;

use crate::cli::fbsim::{
    FbsimCli,
    FbsimSubcommand
};
use crate::cli::game::FbsimGameSubcommand;
use crate::cli::league::FbsimLeagueSubcommand;
use crate::cli::league::team::FbsimLeagueTeamSubcommand;
use crate::cli::league::season::FbsimLeagueSeasonSubcommand;
use crate::cli::league::season::team::FbsimLeagueSeasonTeamSubcommand;

use crate::game::benchmark::game_benchmark;
use crate::game::sim::simulate_game;
use crate::league::create::create_league;
use crate::league::team::add::add_team;
use crate::league::team::get::get_team;
use crate::league::team::list::list_teams;
use crate::league::season::add::add_season;
use crate::league::season::get::get_season;
use crate::league::season::list::list_seasons;
use crate::league::season::team::add::add_season_team;

use clap::Parser;

fn main() {
    // Parse the command-line args
    let fbdb_cli = FbsimCli::parse();

    // Perform the subcommand
    let command = fbdb_cli.command();
    let command_res = match &command {
        FbsimSubcommand::Game { command } => match command {
            FbsimGameSubcommand::Sim(args) => Ok(simulate_game(args.clone())),
            FbsimGameSubcommand::Benchmark(args) => Ok(game_benchmark(args.clone()))
        },
        FbsimSubcommand::League { command } => match command {
            FbsimLeagueSubcommand::Create(args) => Ok(create_league(args.clone())),
            FbsimLeagueSubcommand::Team { command } => match command {
                FbsimLeagueTeamSubcommand::Add(args) => Ok(add_team(args.clone())),
                FbsimLeagueTeamSubcommand::Get(args) => Ok(get_team(args.clone())),
                FbsimLeagueTeamSubcommand::List(args) => Ok(list_teams(args.clone()))
            },
            FbsimLeagueSubcommand::Season { command } => match command {
                FbsimLeagueSeasonSubcommand::Add(args) => add_season(args.clone()),
                FbsimLeagueSeasonSubcommand::Get(args) => Ok(get_season(args.clone())),
                FbsimLeagueSeasonSubcommand::List(args) => Ok(list_seasons(args.clone())),
                FbsimLeagueSeasonSubcommand::Team{ command } => match command {
                    FbsimLeagueSeasonTeamSubcommand::Add(args) => add_season_team(args.clone())
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
