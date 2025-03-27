use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use tokio::sync::RwLockReadGuard;
use uuid::Uuid;
use crate::graphql::queries::{get_games::{self, GetGamesGames}, get_heroes, get_matches::GetMatchesMatches, get_tournament::{self, GetTournamentTournament}, get_users::GetUsersUsers, update_game, GetGames};

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

#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
#[repr(i32)]
pub enum ModType {
    Universe = 1,
    Hrta = 2
}

impl Into<get_heroes::ModType> for ModType {
    fn into(self) -> get_heroes::ModType {
        match self {
            ModType::Hrta => get_heroes::ModType::HRTA,
            ModType::Universe => get_heroes::ModType::UNIVERSE
        }
    }
}

impl Into<ModType> for get_tournament::ModType {
    fn into(self) -> ModType {
        match self {
            get_tournament::ModType::HRTA => ModType::Hrta,
            get_tournament::ModType::UNIVERSE => ModType::Universe,
            _=> unreachable!()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GameType {
    Rmg,
    Arena
}

impl Into<GameType> for get_tournament::GameType {
    fn into(self) -> GameType {
        match self {
            get_tournament::GameType::ARENA => GameType::Arena,
            get_tournament::GameType::RMG => GameType::Rmg,
            _=> unreachable!()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[repr(i32)]
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

impl From<&get_games::GameResult> for GameResult {
    fn from(value: &get_games::GameResult) -> Self {
        match value {
            get_games::GameResult::FIRST_PLAYER_WON => GameResult::FirstPlayerWon,
            get_games::GameResult::SECOND_PLAYER_WON => GameResult::SecondPlayerWon,
            get_games::GameResult::NOT_SELECTED => GameResult::NotSelected,
            _=> unreachable!()
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

impl From<&get_games::GameOutcome> for GameOutcome {
    fn from(value: &get_games::GameOutcome) -> Self {
        match value {
            get_games::GameOutcome::FINAL_BATTLE_VICTORY => GameOutcome::FinalBattleVictory,
            get_games::GameOutcome::NEUTRALS_VICTORY => GameOutcome::NeutralsVictory,
            get_games::GameOutcome::OPPONENT_SURRENDER => GameOutcome::OpponentSurrender,
            _=> unreachable!()
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

impl From<&get_games::BargainsColor> for BargainsColor {
    fn from(value: &get_games::BargainsColor) -> Self {
        match value {
            get_games::BargainsColor::BARGAINS_COLOR_BLUE => BargainsColor::BargainsColorBlue,
            get_games::BargainsColor::BARGAINS_COLOR_RED => BargainsColor::BargainsColorRed,
            get_games::BargainsColor::NOT_SELECTED => BargainsColor::NotSelected,
            _=> unreachable!()
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
#[derive(Debug, Serialize, Deserialize)]
pub struct TournamentFrontendModel {
    pub id: Uuid,
    pub name: String,
    pub mod_type: ModType,
    pub game_type: GameType,
    pub with_bargains: bool,
    pub with_bargains_color: bool,
    pub with_foreign_heroes: bool
}

impl From<GetTournamentTournament> for TournamentFrontendModel {
    fn from(value: GetTournamentTournament) -> Self {
        TournamentFrontendModel {
            id: value.id,
            name: value.name,
            mod_type: value.mod_type.into(),
            game_type: value.game_type.into(),
            with_bargains: value.with_bargains,
            with_bargains_color: value.with_bargains_color,
            with_foreign_heroes: value.with_foreign_heroes
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchFrontendModel {
    pub id: Uuid,
    pub first_user_id: Uuid,
    pub first_user_nickname: String,
    pub second_user_id: Uuid,
    pub second_user_nickname: String
}

impl GetMatchesMatches {
    pub fn into_frontend_model(&self, users_data: &RwLockReadGuard<'_, Vec<GetUsersUsers>>) -> Result<MatchFrontendModel, crate::error::Error> {
        let first_user_nickname = users_data.iter()
            .find(|u| u.id == self.first_player)
            .ok_or(crate::error::Error::Other("Unknown user id".to_string()))?
            .nickname
            .clone();

        let second_user_nickname = users_data.iter()
            .find(|u| u.id == self.second_player)
            .ok_or(crate::error::Error::Other("Unknown user id".to_string()))?
            .nickname
            .clone();

        Ok(MatchFrontendModel { 
            id: self.id, 
            first_user_id: self.first_player, 
            first_user_nickname: first_user_nickname, 
            second_user_id: self.second_player, 
            second_user_nickname: second_user_nickname 
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameFrontendModel {
    pub id: Uuid,
    pub first_player_race: i64,
    pub first_player_hero: i64,
    pub second_player_race: i64,
    pub second_player_hero: i64,
    pub bargains_color: Option<BargainsColor>,
    pub bargains_amount: i64,
    pub result: GameResult,
    pub outcome: GameOutcome
}

impl GetGamesGames {
    pub fn into_frontend_model(&self) -> GameFrontendModel {
        let first_player_race = self.first_player_race.unwrap_or(-1);
        let first_player_hero = self.first_player_hero.unwrap_or(-1);
        let second_player_race = self.second_player_race.unwrap_or(-1);
        let second_player_hero = self.second_player_hero.unwrap_or(-1);
        let bargains_color;
        if let Some(color) = &self.bargains_color {
            bargains_color = Some(BargainsColor::from(color));
        } else {
            bargains_color = None;
        }
        let bargains_amount = self.bargains_amount.unwrap_or(-1);

        GameFrontendModel { 
            id: self.id, 
            first_player_race: first_player_race, 
            first_player_hero: first_player_hero, 
            second_player_race: second_player_race, 
            second_player_hero: second_player_hero, 
            bargains_color: bargains_color, 
            bargains_amount: bargains_amount, 
            result: GameResult::from(&self.result), 
            outcome: GameOutcome::from(&self.outcome) 
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeroFrontendModel {
    pub id: i64,
    pub name: String
}