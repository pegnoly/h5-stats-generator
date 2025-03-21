use std::str::FromStr;

use itertools::Itertools;
use tauri::State;
use uuid::Uuid;

use crate::{
    graphql::queries::{
        get_games::GetGamesGames, get_matches::GetMatchesMatches,
        get_tournaments::GetTournamentsTournamentsAll,
    },
    services::tournament::{
        payloads::UpdateGamePayload,
        service::TournamentService,
        types::{BargainsColor, GameOutcome, GameResult, MatchFrontendModel, ModType, TournamentFrontendModel},
    },
};

use super::manager::AppManager;

#[tauri::command]
pub async fn load_tournaments_list(
    tournament_service: State<'_, TournamentService>,
) -> Result<Vec<GetTournamentsTournamentsAll>, crate::error::Error> {
    let tournaments = tournament_service.get_all_tournaments().await?;
    Ok(tournaments)
}

#[tauri::command]
pub async fn load_tournament(
    tournament_service: State<'_, TournamentService>,
    tournament_id: Uuid,
) -> Result<TournamentFrontendModel, crate::error::Error> {
    let result = tournament_service.get_tournament(tournament_id).await?;
    //println!("Tournament found: {:?}", &result);
    if let Some(tournament) = result {
        Ok(TournamentFrontendModel::from(tournament))
    } else {
        Err(crate::error::Error::Other(format!("No tournament found with id {}", tournament_id)))
    }
}

#[tauri::command]
pub async fn load_heroes(
    tournament_service: State<'_, TournamentService>,
    app_manager: State<'_, AppManager>,
    mod_type: ModType
) -> Result<(), crate::error::Error> {
    //let mod_type = ModType::from_str(&mod_type)?;
    let heroes = tournament_service.get_heroes(mod_type).await?;
    //println!("Heroes: {:?}", &heroes);
    let mut current_heroes_locked = app_manager.current_heroes.write().await;
    *current_heroes_locked = heroes;
    Ok(())
}

#[tauri::command]
pub async fn load_users(
    tournament_service: State<'_, TournamentService>,
    app_manager: State<'_, AppManager>,
    tournament_id: Uuid,
) -> Result<(), crate::error::Error> {
    let users = tournament_service.get_users(tournament_id).await?;
    let mut current_users_locked = app_manager.current_users.write().await;
    *current_users_locked = users;
    Ok(())
}

#[tauri::command]
pub async fn load_matches(
    tournament_service: State<'_, TournamentService>,
    app_manager: State<'_, AppManager>,
    tournament_id: Uuid,
    user_id: Option<Uuid>,
) -> Result<Vec<MatchFrontendModel>, crate::error::Error> {
    let users_data = app_manager.current_users.read().await;
    let matches = tournament_service.get_matches(tournament_id, user_id).await?.into_iter()
        .filter_map(|m| {
            if let Ok(converted_match) = m.into_frontend_model(&users_data) {
                Some(converted_match)
            } else {
                None
            }
        })
        .collect::<Vec<MatchFrontendModel>>();
    Ok(matches)
}

#[tauri::command]
pub async fn load_games(
    tournament_service: State<'_, TournamentService>,
    match_id: Uuid,
) -> Result<Vec<GetGamesGames>, crate::error::Error> {
    Ok(tournament_service.get_games(match_id).await?)
}

#[tauri::command]
pub async fn update_game_first_player_race(
    tournament_service: State<'_, TournamentService>,
    game_id: Uuid,
    race: i64,
) -> Result<(), crate::error::Error> {
    let payload = UpdateGamePayload::new(game_id).with_first_player_race(race);
    Ok(tournament_service.update_game(payload).await?)
}

#[tauri::command]
pub async fn update_game_first_player_hero(
    tournament_service: State<'_, TournamentService>,
    game_id: Uuid,
    hero: i64,
) -> Result<(), crate::error::Error> {
    let payload = UpdateGamePayload::new(game_id).with_first_player_hero(hero);
    Ok(tournament_service.update_game(payload).await?)
}

#[tauri::command]
pub async fn update_game_second_player_race(
    tournament_service: State<'_, TournamentService>,
    game_id: Uuid,
    race: i64,
) -> Result<(), crate::error::Error> {
    let payload = UpdateGamePayload::new(game_id).with_second_player_race(race);
    Ok(tournament_service.update_game(payload).await?)
}

#[tauri::command]
pub async fn update_game_second_player_hero(
    tournament_service: State<'_, TournamentService>,
    game_id: Uuid,
    hero: i64,
) -> Result<(), crate::error::Error> {
    let payload = UpdateGamePayload::new(game_id).with_second_player_hero(hero);
    Ok(tournament_service.update_game(payload).await?)
}

#[tauri::command]
pub async fn update_game_bargains_color(
    tournament_service: State<'_, TournamentService>,
    game_id: Uuid,
    color: BargainsColor,
) -> Result<(), crate::error::Error> {
    let payload = UpdateGamePayload::new(game_id).with_bargains_color(color);
    Ok(tournament_service.update_game(payload).await?)
}

#[tauri::command]
pub async fn update_game_bargains_amount(
    tournament_service: State<'_, TournamentService>,
    game_id: Uuid,
    amount: i64,
) -> Result<(), crate::error::Error> {
    let payload = UpdateGamePayload::new(game_id).with_bargains_amount(amount);
    Ok(tournament_service.update_game(payload).await?)
}

#[tauri::command]
pub async fn update_game_result(
    tournament_service: State<'_, TournamentService>,
    game_id: Uuid,
    result: GameResult,
) -> Result<(), crate::error::Error> {
    let payload = UpdateGamePayload::new(game_id).with_result(result);
    Ok(tournament_service.update_game(payload).await?)
}

#[tauri::command]
pub async fn update_game_outcome(
    tournament_service: State<'_, TournamentService>,
    game_id: Uuid,
    outcome: GameOutcome,
) -> Result<(), crate::error::Error> {
    let payload = UpdateGamePayload::new(game_id).with_outcome(outcome);
    Ok(tournament_service.update_game(payload).await?)
}
