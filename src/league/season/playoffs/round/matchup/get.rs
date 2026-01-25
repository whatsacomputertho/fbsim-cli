use std::fs;

use fbsim_core::league::League;

use crate::cli::league::season::playoffs::round::matchup::FbsimLeagueSeasonPlayoffsRoundMatchupGetArgs;

use serde_json;

pub fn get_playoffs_matchup(args: FbsimLeagueSeasonPlayoffsRoundMatchupGetArgs) -> Result<(), String> {
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

    // Get the playoffs
    let playoffs = season.playoffs();
    let rounds = playoffs.rounds();

    if rounds.is_empty() {
        return Err(format!("Playoffs have not been generated for the {} season", args.year));
    }

    // Get the round
    let round = match rounds.get(args.round) {
        Some(r) => r,
        None => return Err(format!("No playoff round found with index: {}", args.round)),
    };

    // Get the matchup
    let matchup = match round.matchups().get(args.matchup) {
        Some(m) => m,
        None => return Err(format!("No matchup found with index: {}", args.matchup)),
    };

    // Get team names
    let away_team = season.team(*matchup.away_team()).unwrap();
    let home_team = season.team(*matchup.home_team()).unwrap();
    let context = matchup.context();

    // Display matchup info
    println!("Playoff Round {} Matchup {}", args.round, args.matchup);
    println!();
    println!("{} @ {}", away_team.name(), home_team.name());
    println!();
    println!("{}", context);

    // Display stats if game is complete
    if context.game_over() {
        if let Some(home_stats) = matchup.home_stats() {
            println!();
            println!("{} stats\n{}", context.home_team_short(), home_stats);
        }
        if let Some(away_stats) = matchup.away_stats() {
            println!();
            println!("{} stats\n{}", context.away_team_short(), away_stats);
        }
    }

    Ok(())
}
