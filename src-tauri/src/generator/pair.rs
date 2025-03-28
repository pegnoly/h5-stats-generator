use std::collections::HashMap;

use ordered_float::OrderedFloat;
use rust_xlsxwriter::{workbook::Workbook, worksheet::Worksheet, Format};

use crate::services::tournament::types::GameResult;

use super::{styles::{Style, STYLES}, types::GameEntry, RaceInfo, TournamentStatsModel};

pub struct PairStatsBuilder {
    wins_by_race: HashMap<i64, HashMap<i64, usize>>,
    losses_by_race: HashMap<i64, HashMap<i64, usize>>,
    mirrors_by_race: HashMap<i64, usize>
}

impl PairStatsBuilder {
    pub fn new() -> Self {

        let races_range = std::ops::Range {start: 0, end: 9};

        PairStatsBuilder { 
            wins_by_race: HashMap::from_iter(
                races_range.clone().map(|r| {
                    (r, HashMap::from_iter(races_range.clone().map(|r2| {
                            (r2, 0)
                    })))
                })), 

            losses_by_race: HashMap::from_iter(
                races_range.clone().map(|r| {
                    (r, HashMap::from_iter(races_range.clone().map(|r2| {
                            (r2, 0)
                    })))
                })),

            mirrors_by_race: HashMap::from_iter(
                races_range.clone().map(|r| {
                    (r, 0)
                })
            )
        }
    }

    pub fn build(&mut self, model: &TournamentStatsModel, workbook: &mut Workbook) -> Result<(), crate::error::Error> {
        let worksheet = workbook.add_worksheet().set_name("Общая статистика по расам")?;
        self.build_pairs_win_loss_stats(&model.races, &model.games, worksheet)?;
        self.build_total_games_and_winrates(&model.races, worksheet)?;
        self.build_match_ups_games_and_winrates(&model.races, worksheet)?;
        Ok(())
    }

    fn build_pairs_win_loss_stats(&mut self, races_data: &Vec<RaceInfo>, games_data: &Vec<GameEntry>, worksheet: &mut Worksheet) -> Result<(), crate::error::Error> {
        let width = races_data.iter()
            .map(|r| r.name.clone())
            .collect::<Vec<String>>().iter()
            .max_by_key(|x| x.len()).ok_or(crate::error::Error::Other("Max by key error".to_string()))?
            .chars().count();

        worksheet.merge_range(0, 0, 1, 0, "VS", STYLES.get(&Style::BackgroundRed)?)?;
        worksheet.set_column_width(0, (width + 1) as f64)?;

        for race in races_data {
            match race.id {
                0 => {},
                _=> {
                    worksheet.write_with_format(1 + (race.id as u32), 0, &race.name, STYLES.get(&Style::ThinBorderTextWrap)?)?;
                    let col_offset = (race.id as u16) * 2 - 1;
                    worksheet.merge_range(
                        0, 
                        col_offset, 
                        0, 
                        col_offset + 1,
                        &race.name, 
                        STYLES.get(&Style::ThinBorderTextCenter)?
                    )?;
                    worksheet.set_column_width(col_offset, (width as f64) / 1.5)?;
                    worksheet.set_column_width(col_offset + 1, (width as f64) / 1.5)?;
                    worksheet.write_with_format(1, col_offset, "Побед", STYLES.get(&Style::ThinBorderTextCenter)?)?;
                    worksheet.write_with_format(1, col_offset + 1, "Поражений", STYLES.get(&Style::ThinBorderTextCenter)?)?;

                    for opponent_race in races_data {
                        match opponent_race.id {
                            0 => {},
                            _=> {
                                let row_offset = (opponent_race.id as u32) + 1;
                                if race.id != opponent_race.id {
                                    // check for games where either 1-2 pair is current race-opponent(and 1st won) or 1-2 is opponent-race(and 2nd won)
                                    let wins = games_data.iter().filter(|game| {
                                            (game.first_player_race == opponent_race.id && game.second_player_race == race.id && game.result == GameResult::FirstPlayerWon) ||
                                            (game.first_player_race == race.id && game.second_player_race == opponent_race.id && game.result == GameResult::SecondPlayerWon)
                                        })
                                        .collect::<Vec<&GameEntry>>()
                                        .len();
                                    
                                    if let Some(opponent_race_data) = self.wins_by_race.get_mut(&opponent_race.id) {
                                        if let Some(current_wins) = opponent_race_data.get_mut(&race.id) {
                                            *current_wins = wins;
                                        }
                                    }

                                    let losses = games_data.iter().filter(|game| {
                                            (game.first_player_race == opponent_race.id && game.second_player_race == race.id && game.result == GameResult::SecondPlayerWon) ||
                                            (game.first_player_race == race.id && game.second_player_race == opponent_race.id && game.result == GameResult::FirstPlayerWon)
                                        })
                                        .collect::<Vec<&GameEntry>>()
                                        .len();

                                    if let Some(opponent_race_data) = self.losses_by_race.get_mut(&opponent_race.id) {
                                        if let Some(current_losses) = opponent_race_data.get_mut(&race.id) {
                                            *current_losses = losses;
                                        }
                                    }

                                    worksheet.write_with_format(row_offset, col_offset, wins as u32, STYLES.get(&Style::ThinBorderTextWrap)?)?;
                                    worksheet.write_with_format(row_offset, col_offset + 1, losses as u32, STYLES.get(&Style::ThinBorderTextWrap)?)?;
                                }
                                else {
                                    // mirrors
                                    let games_count = games_data.iter()
                                        .filter(|game| {
                                            game.first_player_race == race.id && game.first_player_race == game.second_player_race
                                        })
                                        .collect::<Vec<&GameEntry>>()
                                        .len();
                                    
                                    if let Some(mirrors_count) = self.mirrors_by_race.get_mut(&race.id) {
                                        *mirrors_count = games_count;
                                    }
                                    
                                    worksheet.merge_range(
                                        row_offset, 
                                        col_offset, 
                                        row_offset, 
                                        col_offset + 1, 
                                        &format!("{}", games_count), STYLES.get(&Style::ThinBorderTextWrap)?)?;
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn build_total_games_and_winrates(&mut self, races_data: &Vec<RaceInfo>, worksheet: &mut Worksheet) -> Result<(), crate::error::Error> {
        worksheet
            .write_with_format(0, 17, "Всего игр", STYLES.get(&Style::ThinBorderTextCenter)?)?
            .set_cell_format(1, 17, STYLES.get(&Style::BackgroundSilver)?)?;

        worksheet.merge_range(11, 0, 11, 1, "Общий винрейт", STYLES.get(&Style::ThinBorderTextCenter)?)?;

        let races_total_games = races_data.iter()
            .map(|r| {
                (r.id, 
                    calc_games(self.losses_by_race.get(&r.id).unwrap()) + 
                    calc_games(self.wins_by_race.get(&r.id).unwrap()) + 
                    self.mirrors_by_race.get(&r.id).unwrap()
                )
            })
            .collect::<HashMap<i64, usize>>();

        let races_total_games_no_mirrors = races_data.iter()
            .map(|r| {
                (r.id, 
                    calc_games(self.losses_by_race.get(&r.id).unwrap()) + 
                    calc_games(self.wins_by_race.get(&r.id).unwrap())
                )
            })
            .collect::<HashMap<i64, usize>>();

        let least_played_race = races_total_games.iter()
            .min_by_key(|r| r.1)
            .unwrap()
            .0;

        let most_played_race = races_total_games.iter()
            .max_by_key(|r| r.1)
            .unwrap()
            .0;

        let races_winrates = races_data.iter()
            .map(|r| {
                (r.id, (calc_games(self.wins_by_race.get(&r.id).unwrap()) as f32) / (*races_total_games_no_mirrors.get(&r.id).unwrap() as f32) * 100.0)
            })
            .collect::<HashMap<i64, f32>>();

        let race_with_least_winrate = races_winrates.iter()
            .min_by_key(|r| OrderedFloat(*r.1))
            .unwrap()
            .0;

        let race_with_most_winrate = races_winrates.iter()
            .max_by_key(|r| OrderedFloat(*r.1))
            .unwrap()
            .0;

        for race in races_data {
            match race.id {
                0 => {},
                _=> {
                    let row_offset = 1 + (race.id as u32);
                    worksheet.write_with_format(row_offset, 17, *races_total_games.get(&race.id).unwrap() as u32, STYLES.get(&Style::ThinBorderTextWrap)?)?;
                    let row_offset = 11 + (race.id as u32);
                    worksheet.write_with_format(row_offset, 0, &race.name, STYLES.get(&Style::ThinBorderTextWrap)?)?;
                    worksheet.write_with_format(row_offset, 1, &format!("{:.3}%", *races_winrates.get(&race.id).unwrap()), STYLES.get(&Style::ThinBorderTextWrap)?)?;
                }
            }
        }

        worksheet
            .set_cell_format(1 + (*most_played_race as u32), 17, STYLES.get(&Style::BackgroundGreen)?)?
            .set_cell_format(1 + (*least_played_race as u32), 17, STYLES.get(&Style::BackgroundRed)?)?
            .set_cell_format(11 + (*race_with_most_winrate as u32), 1, STYLES.get(&Style::BackgroundGreen)?)?
            .set_cell_format(11 + (*race_with_least_winrate as u32), 1, STYLES.get(&Style::BackgroundRed)?)?;
        Ok(())
    }

    fn build_match_ups_games_and_winrates(&mut self, races_data: &Vec<RaceInfo>, worksheet: &mut Worksheet) -> Result<(), crate::error::Error> {
        let mut most_played_pair_first = 0;
        let mut most_played_pair_second = 0;
        
        let mut least_played_pair_first = 0;
        let mut least_played_pair_second = 0;
    
        let mut most_played_pair_games = u32::MIN;
        let mut least_played_pair_games = u32::MAX;
    
        worksheet.merge_range(21, 3, 21, 6, "Число игр по матчапам", 
            &Format::new().set_align(rust_xlsxwriter::FormatAlign::Center).set_align(rust_xlsxwriter::FormatAlign::CenterAcross).set_bold())?;
    
        worksheet.merge_range(33, 3, 33, 6, "Винрейты матчапов", 
        &Format::new().set_align(rust_xlsxwriter::FormatAlign::Center).set_align(rust_xlsxwriter::FormatAlign::CenterAcross).set_bold())?;
    
        for race in races_data {
            match race.id {
                0 => {},
                _=> {
                    let col_offset = race.id as u16;
                    let games_row_offset = 23 + (race.id as u32);
                    let winrate_row_offset = 35 + (race.id as u32);
    
                    worksheet.write_with_format(games_row_offset, 0, &race.name, STYLES.get(&Style::ThinBorderTextWrap)?)?;
                    worksheet.write_with_format(23, col_offset, &race.name, STYLES.get(&Style::ThinBorderTextWrap)?)?;
    
                    worksheet.write_with_format(winrate_row_offset, 0, &race.name, STYLES.get(&Style::ThinBorderTextWrap)?)?;
                    worksheet.write_with_format(35, col_offset, &race.name, STYLES.get(&Style::ThinBorderTextWrap)?)?;
    
                    for opponent_race in races_data {
                        match opponent_race.id {
                            0 => {},
                            _=> {
                                let col_offset = opponent_race.id as u16;
                                if race.id == opponent_race.id {
                                    worksheet.set_cell_format(games_row_offset, col_offset, STYLES.get(&Style::BackgroundBlack)?)?;
                                    worksheet.set_cell_format(winrate_row_offset, col_offset, STYLES.get(&Style::BackgroundBlack)?)?;
                                }
                                else {
                                    let pair_wins = *self.wins_by_race.get(&race.id).unwrap().get(&opponent_race.id).unwrap();
                                    let pair_losses = *self.losses_by_race.get(&race.id).unwrap().get(&opponent_race.id).unwrap();
                                    let total_pair_games = (pair_wins + pair_losses) as u32;
    
                                    let pair_winrate = (pair_wins as f32) / (total_pair_games as f32) * 100.0;
                                    worksheet.write_with_format(games_row_offset, col_offset, total_pair_games as u32, STYLES.get(&Style::ThinBorderTextWrap)?)?;
                                    worksheet.write_with_format(
                                        winrate_row_offset, 
                                        col_offset, 
                                        format!("{:.3}%", pair_winrate as f32), 
                                        STYLES.get(&Style::ThinBorderTextWrap)?
                                    )?;
    
                                    if total_pair_games > most_played_pair_games {
                                        most_played_pair_games = total_pair_games;
                                        most_played_pair_first = opponent_race.id;
                                        most_played_pair_second = race.id;
                                    }
    
                                    if total_pair_games < least_played_pair_games {
                                        least_played_pair_games = total_pair_games;
                                        least_played_pair_first = opponent_race.id;
                                        least_played_pair_second = race.id;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // MOST - LEAST PLAYED PAIRS
        worksheet.set_cell_format(23 + (most_played_pair_first as u32), most_played_pair_second as u16, STYLES.get(&Style::BackgroundGreen)?)?;
        worksheet.set_cell_format(23 + (most_played_pair_second as u32), most_played_pair_first as u16, STYLES.get(&Style::BackgroundGreen)?)?;
        worksheet.set_cell_format(23 + (least_played_pair_first as u32), least_played_pair_second as u16, STYLES.get(&Style::BackgroundRed)?)?;
        worksheet.set_cell_format(23 + (least_played_pair_second as u32), least_played_pair_first as u16, STYLES.get(&Style::BackgroundRed)?)?;
        Ok(())
    }
}

fn calc_games(data: &HashMap<i64, usize>) -> usize {
    data.into_iter().map(|d| *d.1).sum()
}