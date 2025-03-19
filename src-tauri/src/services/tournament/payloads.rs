use crate::graphql::queries::update_game;
use uuid::Uuid;

use super::types;

#[derive(Debug, Default)]
pub struct UpdateGamePayload {
    pub id: Uuid,
    pub first_player_race: Option<i64>,
    pub first_player_hero: Option<i64>,
    pub second_player_race: Option<i64>,
    pub second_player_hero: Option<i64>,
    pub bargains_color: Option<update_game::BargainsColor>,
    pub bargains_amount: Option<i64>,
    pub result: Option<update_game::GameResult>,
    pub outcome: Option<update_game::GameOutcome>,
}

impl UpdateGamePayload {
    pub fn new(id: Uuid) -> Self {
        UpdateGamePayload {
            id: id,
            ..Default::default()
        }
    }

    pub fn with_first_player_race(mut self, race: i64) -> Self {
        self.first_player_race = Some(race);
        self
    }

    pub fn with_first_player_hero(mut self, hero: i64) -> Self {
        self.first_player_hero = Some(hero);
        self
    }

    pub fn with_second_player_race(mut self, race: i64) -> Self {
        self.second_player_race = Some(race);
        self
    }

    pub fn with_second_player_hero(mut self, hero: i64) -> Self {
        self.second_player_hero = Some(hero);
        self
    }

    pub fn with_bargains_color(mut self, color: types::BargainsColor) -> Self {
        self.bargains_color = Some(color.into());
        self
    }

    pub fn with_bargains_amount(mut self, amount: i64) -> Self {
        self.bargains_amount = Some(amount);
        self
    }

    pub fn with_result(mut self, result: types::GameResult) -> Self {
        self.result = Some(result.into());
        self
    }

    pub fn with_outcome(mut self, outcome: types::GameOutcome) -> Self {
        self.outcome = Some(outcome.into());
        self
    }
}

impl Into<update_game::Variables> for UpdateGamePayload {
    fn into(self) -> update_game::Variables {
        update_game::Variables {
            id: self.id,
            first_player_race: self.first_player_race,
            first_player_hero: self.first_player_hero,
            second_player_race: self.second_player_race,
            second_player_hero: self.second_player_hero,
            bargains_color: self.bargains_color,
            bargains_amount: self.bargains_amount,
            result: self.result,
            outcome: self.outcome,
        }
    }
}
