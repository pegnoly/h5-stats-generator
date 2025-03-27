use app::{commands::{
    get_heroes_of_race, load_games, load_heroes, load_matches, load_tournament, load_tournaments_list, load_users, select_game, update_game_bargains_amount, update_game_bargains_color, update_game_first_player_hero, update_game_first_player_race, update_game_outcome, update_game_result, update_game_second_player_hero, update_game_second_player_race
}, manager::AppManager};
use generator::commands::invoke_generation;
use services::tournament::service::TournamentService;
use tokio::sync::{Mutex, RwLock};

pub mod app;
pub mod generator;
pub mod graphql;
pub mod services;
pub mod error;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(TournamentService::new())
        .manage(AppManager {current_heroes: RwLock::new(vec![]), current_users: RwLock::new(vec![]), current_games: RwLock::new(vec![])})
        .invoke_handler(tauri::generate_handler![
            load_tournaments_list,
            load_tournament,
            load_heroes,
            load_users,
            load_matches,
            load_games,
            select_game,
            get_heroes_of_race,
            update_game_first_player_race,
            update_game_first_player_hero,
            update_game_second_player_race,
            update_game_second_player_hero,
            update_game_bargains_color,
            update_game_bargains_amount,
            update_game_result,
            update_game_outcome,
            invoke_generation
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
