use rust_xlsxwriter::worksheet::Worksheet;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::Error, graphql::queries::{get_all_games, get_tournament::{self, GetTournamentTournament}}, services::tournament::types::{BargainsColor, GameOutcome, GameResult}};

use super::styles::{Style, STYLES};

pub struct PlayerMatchHistoryHeaders<'a> {
    pub headers: Vec<&'a str>
}

impl<'a> PlayerMatchHistoryHeaders<'a> {
    pub fn new(tournament_info: &GetTournamentTournament) -> Self {
        let mut headers = vec!["Фракция игрока", "Фракция оппонента", "Герой игрока", "Герой оппонента"];
        if tournament_info.with_bargains {
            headers.push("Торг игрока");
        }
        if tournament_info.with_bargains_color {
            headers.push("Цвет торга");
        }
        headers.push("Результат");
        // if tournament_info.game_type == get_tournament::GameType::RMG {
        //     headers.push("Исход");
        // }
        PlayerMatchHistoryHeaders { headers: headers }
    }

    pub fn to_xlsx(&self, worksheet: &mut Worksheet) -> Result<(), Error> {
        worksheet.merge_range(0, 0, 0, self.headers.len() as u16, "История игр", STYLES.get(&Style::TextBoldCentered)?)?;
        worksheet.set_column_width(0, 14)?;
        worksheet.write_with_format(1, 0, "VS", STYLES.get(&Style::TextCenterColorRed)?)?;
        let mut col_offset = 1;
        for cell_name in &self.headers {
            worksheet.set_column_width(col_offset, 14)?;
            worksheet.write_with_format(1, col_offset, *cell_name, STYLES.get(&Style::ThinBorderTextWrap)?)?;
            col_offset += 1;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ResultOutput {
    Win,
    Loss
}

#[derive(Debug)]
pub struct GameHistoryEntry<'a> {
    pub opponent: &'a String,
    pub player_race: &'a String,
    pub player_hero: &'a String,
    pub opponent_race: &'a String,
    pub opponent_hero: &'a String,
    pub bargains_amount: Option<i64>,
    pub bargains_color: Option<&'a str>,
    pub result: ResultOutput,
    pub outcome: Option<&'a String>
}

impl<'a> GameHistoryEntry<'a> {
    pub fn to_xlsx(&self, worksheet: &mut Worksheet, row: u32) -> Result<(), Error> {
        let mut col = 0;
        worksheet.write_with_format(row, col, self.opponent, STYLES.get(&Style::ThinBorderTextWrap)?)?;
        col+=1;
        worksheet.write_with_format(row, col, self.player_race, STYLES.get(&Style::ThinBorderTextWrap)?)?;
        col+=1;
        worksheet.write_with_format(row, col, self.player_hero, STYLES.get(&Style::ThinBorderTextWrap)?)?;
        col+=1;
        worksheet.write_with_format(row, col, self.opponent_race, STYLES.get(&Style::ThinBorderTextWrap)?)?;
        col+=1;
        worksheet.write_with_format(row, col, self.opponent_hero, STYLES.get(&Style::ThinBorderTextWrap)?)?;
        col+=1;
        if let Some(bargains_amount) = self.bargains_amount {
            worksheet.write_with_format(row, col, bargains_amount, STYLES.get(&Style::ThinBorderTextWrap)?)?;
            col+=1;
        }
        if let Some(bargains_color) = self.bargains_color {
            println!("Bargains color: {bargains_color}");
            worksheet.write_with_format(row, col, bargains_color, STYLES.get(&Style::ThinBorderTextWrap)?)?;
            col+=1;
        };
        match self.result {
            ResultOutput::Win => {
                worksheet.write_with_format(row, col, "Победа".to_string(), STYLES.get(&Style::BackgroundGreen)?)?;
            },
            ResultOutput::Loss => {
                worksheet.write_with_format(row, col, "Поражение".to_string(), STYLES.get(&Style::BackgroundRed)?)?;
            }
        }
        col+=1;
        if let Some(outcome) = self.outcome {
            worksheet.write_with_format(row, col, outcome, STYLES.get(&Style::ThinBorderTextWrap)?)?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameEntry {
    pub match_id: Uuid,
    pub first_player_race: i64,
    pub first_player_hero: i64,
    pub second_player_race: i64,
    pub second_player_hero: i64,
    pub bargains_amount: i64,
    pub bargains_color: Option<BargainsColor>,
    pub result: GameResult,
    pub outcome: GameOutcome
}

impl Into<GameResult> for get_all_games::GameResult {
    fn into(self) -> GameResult {
        match self {
            get_all_games::GameResult::FIRST_PLAYER_WON => GameResult::FirstPlayerWon,
            get_all_games::GameResult::SECOND_PLAYER_WON => GameResult::SecondPlayerWon,
            get_all_games::GameResult::NOT_SELECTED => GameResult::NotSelected,
            _=> unreachable!()
        }
    }
}

impl Into<GameOutcome> for get_all_games::GameOutcome {
    fn into(self) -> GameOutcome {
        match self {
            get_all_games::GameOutcome::FINAL_BATTLE_VICTORY => GameOutcome::FinalBattleVictory,
            get_all_games::GameOutcome::NEUTRALS_VICTORY => GameOutcome::NeutralsVictory,
            get_all_games::GameOutcome::OPPONENT_SURRENDER => GameOutcome::OpponentSurrender,
            _=> unreachable!()
        }
    }
}

impl TryFrom<get_all_games::GetAllGamesGamesAll> for GameEntry {
    type Error = crate::error::Error;

    fn try_from(value: get_all_games::GetAllGamesGamesAll) -> Result<Self, Self::Error> {
        let first_player_race = value.first_player_race.ok_or(Error::NoGameField {field: "first_player_race".to_string(), game_id: value.id})?;
        let first_player_hero = value.first_player_hero.ok_or(Error::NoGameField {field: "first_player_hero".to_string(), game_id: value.id})?;
        let second_player_race = value.second_player_race.ok_or(Error::NoGameField {field: "second_player_race".to_string(), game_id: value.id})?;
        let second_player_hero = value.second_player_hero.ok_or(Error::NoGameField {field: "second_player_hero".to_string(), game_id: value.id})?;

        Ok(GameEntry {
            match_id: value.match_id,
            first_player_race,
            first_player_hero,
            second_player_race,
            second_player_hero,
            bargains_amount: value.bargains_amount.unwrap_or(-1),
            bargains_color: if let Some(color) = value.bargains_color {
                match color {
                    get_all_games::BargainsColor::BARGAINS_COLOR_BLUE => Some(BargainsColor::BargainsColorBlue),
                    get_all_games::BargainsColor::BARGAINS_COLOR_RED => Some(BargainsColor::BargainsColorRed),
                    _=> unreachable!()
                }
            } else {
                None
            },
            result: value.result.into(),
            outcome: value.outcome.into()
        })
    }
}