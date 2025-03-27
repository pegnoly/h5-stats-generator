use rust_xlsxwriter::worksheet::Worksheet;

use crate::{error::Error, graphql::queries::get_tournament::{self, GetTournamentTournament}};

use super::styles::{Style, STYLES};

pub struct PlayerMatchHistoryHeaders<'a> {
    pub headers: Vec<&'a str>
}

impl<'a> PlayerMatchHistoryHeaders<'a> {
    pub fn new(tournament_info: &GetTournamentTournament) -> Self {
        let mut headers = vec!["Фракция игрока", "Фракция оппонента", "Герой игрока", "Герой оппонента"];
        if tournament_info.with_bargains == true {
            headers.push("Торг игрока");
        }
        if tournament_info.with_bargains_color == true {
            headers.push("Цвет торга");
        }
        headers.push("Результат");
        if tournament_info.game_type == get_tournament::GameType::RMG {
            headers.push("Исход");
        }
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
    pub bargains_color: Option<&'a String>,
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
