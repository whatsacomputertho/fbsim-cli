mod cli;
mod game;
mod league;

use crate::cli::fbsim::{
    FbsimCli,
    FbsimSubcommand
};
use crate::cli::game::FbsimGameSubcommand;
use crate::cli::league::FbsimLeagueSubcommand;
use crate::cli::league::team::FbsimLeagueTeamSubcommand;

use crate::game::benchmark::game_benchmark;
use crate::game::sim::simulate_game;
use crate::league::create::create_league;
use crate::league::team::add::add_team;
use crate::league::team::get::get_team;
use crate::league::team::list::list_teams;

use clap::Parser;

fn main() {
    // Parse the command-line args
    let fbdb_cli = FbsimCli::parse();

    // Perform the subcommand
    let command = fbdb_cli.command();
    match &command {
        FbsimSubcommand::Game { command } => match command {
            FbsimGameSubcommand::Sim(args) => simulate_game(args.clone()),
            FbsimGameSubcommand::Benchmark(args) => game_benchmark(args.clone())
        },
        FbsimSubcommand::League { command } => match command {
            FbsimLeagueSubcommand::Create(args) => create_league(args.clone()),
            FbsimLeagueSubcommand::Team { command } => match command {
                FbsimLeagueTeamSubcommand::Add(args) => add_team(args.clone()),
                FbsimLeagueTeamSubcommand::Get(args) => get_team(args.clone()),
                FbsimLeagueTeamSubcommand::List(args) => list_teams(args.clone())
            }
        }
    }
}
