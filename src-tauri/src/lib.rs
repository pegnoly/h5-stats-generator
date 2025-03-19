use app::commands::{
    load_games, load_matches, load_tournament, load_tournaments_list, load_users,
    update_game_bargains_amount, update_game_bargains_color, update_game_first_player_hero,
    update_game_first_player_race, update_game_outcome, update_game_result,
    update_game_second_player_hero, update_game_second_player_race,
};
use services::tournament::service::TournamentService;
pub mod app;
pub mod graphql;
pub mod services;
pub mod error;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(TournamentService::new())
        .invoke_handler(tauri::generate_handler![
            load_tournaments_list,
            load_tournament,
            load_users,
            load_matches,
            load_games,
            update_game_first_player_race,
            update_game_first_player_hero,
            update_game_second_player_race,
            update_game_second_player_hero,
            update_game_bargains_color,
            update_game_bargains_amount,
            update_game_result,
            update_game_outcome
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
