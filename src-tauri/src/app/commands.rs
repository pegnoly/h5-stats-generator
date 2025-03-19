use tauri::State;
use uuid::Uuid;

use crate::{
    graphql::queries::{
        get_games::GetGamesGames, get_matches::GetMatchesMatches,
        get_tournament::GetTournamentTournament, get_tournaments::GetTournamentsTournamentsAll,
        get_users::GetUsersUsers,
    },
    services::tournament::{
        payloads::UpdateGamePayload,
        service::TournamentService,
        types::{BargainsColor, GameOutcome, GameResult},
    },
};

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
) -> Result<GetTournamentTournament, crate::error::Error> {
    let result = tournament_service.get_tournament(tournament_id).await?;
    if let Some(tournament) = result {
        Ok(tournament)
    } else {
        Err(crate::error::Error::Other(format!("No tournament found with id {}", tournament_id)))
    }
}

#[tauri::command]
pub async fn load_users(
    tournament_service: State<'_, TournamentService>,
    tournament_id: Uuid,
) -> Result<Vec<GetUsersUsers>, crate::error::Error> {
    Ok(tournament_service.get_users(tournament_id).await.unwrap())
}

#[tauri::command]
pub async fn load_matches(
    tournament_service: State<'_, TournamentService>,
    tournament_id: Uuid,
    user_id: Option<Uuid>,
) -> Result<Vec<GetMatchesMatches>, crate::error::Error> {
    Ok(tournament_service
        .get_matches(tournament_id, user_id)
        .await.unwrap())
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
) -> Result<(), ()> {
    let payload = UpdateGamePayload::new(game_id).with_first_player_race(race);
    Ok(tournament_service.update_game(payload).await.unwrap())
}

#[tauri::command]
pub async fn update_game_first_player_hero(
    tournament_service: State<'_, TournamentService>,
    game_id: Uuid,
    hero: i64,
) -> Result<(), ()> {
    let payload = UpdateGamePayload::new(game_id).with_first_player_hero(hero);
    Ok(tournament_service.update_game(payload).await.unwrap())
}

#[tauri::command]
pub async fn update_game_second_player_race(
    tournament_service: State<'_, TournamentService>,
    game_id: Uuid,
    race: i64,
) -> Result<(), ()> {
    let payload = UpdateGamePayload::new(game_id).with_second_player_race(race);
    Ok(tournament_service.update_game(payload).await.unwrap())
}

#[tauri::command]
pub async fn update_game_second_player_hero(
    tournament_service: State<'_, TournamentService>,
    game_id: Uuid,
    hero: i64,
) -> Result<(), ()> {
    let payload = UpdateGamePayload::new(game_id).with_second_player_hero(hero);
    Ok(tournament_service.update_game(payload).await.unwrap())
}

#[tauri::command]
pub async fn update_game_bargains_color(
    tournament_service: State<'_, TournamentService>,
    game_id: Uuid,
    color: BargainsColor,
) -> Result<(), ()> {
    let payload = UpdateGamePayload::new(game_id).with_bargains_color(color);
    Ok(tournament_service.update_game(payload).await.unwrap())
}

#[tauri::command]
pub async fn update_game_bargains_amount(
    tournament_service: State<'_, TournamentService>,
    game_id: Uuid,
    amount: i64,
) -> Result<(), ()> {
    let payload = UpdateGamePayload::new(game_id).with_bargains_amount(amount);
    Ok(tournament_service.update_game(payload).await.unwrap())
}

#[tauri::command]
pub async fn update_game_result(
    tournament_service: State<'_, TournamentService>,
    game_id: Uuid,
    result: GameResult,
) -> Result<(), ()> {
    let payload = UpdateGamePayload::new(game_id).with_result(result);
    Ok(tournament_service.update_game(payload).await.unwrap())
}

#[tauri::command]
pub async fn update_game_outcome(
    tournament_service: State<'_, TournamentService>,
    game_id: Uuid,
    outcome: GameOutcome,
) -> Result<(), ()> {
    let payload = UpdateGamePayload::new(game_id).with_outcome(outcome);
    Ok(tournament_service.update_game(payload).await.unwrap())
}
