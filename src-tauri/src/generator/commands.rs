use rust_xlsxwriter::workbook::Workbook;
use tauri::State;
use uuid::Uuid;
use crate::error::Error as Error;

use crate::generator::pair::PairStatsBuilder;
use crate::generator::race::RaceStatsBuilder;
use crate::generator::types::GameEntry;
use crate::services::tournament::service::TournamentService;

use super::player::build_player_stats;
use super::TournamentStatsModel;

#[tauri::command]
pub async fn invoke_generation(
    tournament_service: State<'_, TournamentService>,
    tournament_id: Uuid
) -> Result<(), Error> {
    let mut tournament_stats_model = TournamentStatsModel::default();
    let tournament = tournament_service.get_tournament(tournament_id).await?
        .ok_or(Error::Other(format!("No tournament with id {}", tournament_id)))?;
    let users = tournament_service.get_users(tournament_id).await?;
    let matches = tournament_service.get_matches(tournament_id, None).await?;
    let games = tournament_service.get_all_games(tournament_id).await?;
    let games_converted = games.into_iter()
        .filter_map(|game| {
            if let Ok(converted_game) = GameEntry::try_from(game) {
                Some(converted_game)
            } else {
                None
            }
        })
        .collect::<Vec<GameEntry>>();
    let heroes = tournament_service.get_heroes(tournament.mod_type.clone().into()).await?;
    tournament_stats_model.tournament = Some(tournament);
    tournament_stats_model.games = games_converted;
    tournament_stats_model.heroes = heroes;
    tournament_stats_model.users = users;
    tournament_stats_model.matches = matches;

    let mut workbook = Workbook::new();
    let mut pair_stats_builder = PairStatsBuilder::new();
    let mut race_stats_builder = RaceStatsBuilder::new();
    pair_stats_builder.build(&tournament_stats_model, &mut workbook)?;
    race_stats_builder.build(&tournament_stats_model, &mut workbook)?;
    build_player_stats(&tournament_stats_model, &mut workbook)?;
    println!("Done");
    workbook.save("D:\\test.xlsx")?;
    Ok(()) 
}