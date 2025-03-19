use serde::{Deserialize, Serialize};
use crate::graphql::queries::update_game;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Match {
//     pub id: Uuid,
//     pub tournament_id: Uuid,
//     pub first_player: Uuid,
//     pub second_player: Uuid,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct User {
//     pub id: Uuid,
//     pub nickname: String,
// }

#[derive(Debug, Serialize, Deserialize)]
pub enum GameResult {
    NotSelected = 0,
    FirstPlayerWon = 1,
    SecondPlayerWon = 2,
}

impl Into<update_game::GameResult> for GameResult {
    fn into(self) -> update_game::GameResult {
        match self {
            GameResult::FirstPlayerWon => update_game::GameResult::FIRST_PLAYER_WON,
            GameResult::SecondPlayerWon => update_game::GameResult::SECOND_PLAYER_WON,
            GameResult::NotSelected => update_game::GameResult::NOT_SELECTED,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GameOutcome {
    FinalBattleVictory = 0,
    NeutralsVictory = 1,
    OpponentSurrender = 2,
}

impl Into<update_game::GameOutcome> for GameOutcome {
    fn into(self) -> update_game::GameOutcome {
        match self {
            GameOutcome::FinalBattleVictory => update_game::GameOutcome::FINAL_BATTLE_VICTORY,
            GameOutcome::NeutralsVictory => update_game::GameOutcome::NEUTRALS_VICTORY,
            GameOutcome::OpponentSurrender => update_game::GameOutcome::OPPONENT_SURRENDER,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BargainsColor {
    NotSelected = 0,
    BargainsColorRed = 2,
    BargainsColorBlue = 3,
}

impl Into<update_game::BargainsColor> for BargainsColor {
    fn into(self) -> update_game::BargainsColor {
        match self {
            BargainsColor::BargainsColorBlue => update_game::BargainsColor::BARGAINS_COLOR_BLUE,
            BargainsColor::BargainsColorRed => update_game::BargainsColor::BARGAINS_COLOR_RED,
            BargainsColor::NotSelected => update_game::BargainsColor::NOT_SELECTED,
        }
    }
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Game {
//     pub id: Uuid,
//     pub match_id: Uuid,
//     pub first_player_race: i64,
//     pub first_player_hero: i64,
//     pub second_player_race: i64,
//     pub second_player_hero: i64,
//     pub bargains_color: Option<BargainsColor>,
//     pub bargains_amount: i32,
//     pub result: GameResult,
//     pub outcome: GameOutcome,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Tournament {
//     pub id: Uuid,
//     pub name: String,
// }
