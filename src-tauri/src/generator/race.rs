use std::collections::HashMap;

use itertools::Itertools;
use rust_xlsxwriter::{workbook::Workbook, worksheet::Worksheet};

use crate::{error::Error, graphql::queries::get_heroes::GetHeroesHeroesNewHeroesEntities, services::tournament::types::GameResult};

use super::{styles::{Style, STYLES}, types::GameEntry, RaceInfo, TournamentStatsModel};

const BARGAINS_CELLS_NAMES: [&str; 16] = [
    "Игр с плюсом по золоту", 
    "Игр с минусом по золоту",
    "Игр без торга", 
    "Побед с плюсом", 
    "Поражений с плюсом",
    "Винрейт с плюсом",
    "Побед с минусом",
    "Поражений с минусом",
    "Винрейт с минусом",
    "Побед без торга",
    "Поражений без торга",
    "Винрейт без торга",
    "Максимальный плюс по золоту",
    "Максимальный минус по золоту",
    "Средний плюсовый торг",
    "Средний минусовый торг"
];

const PLUS_GOLD_TOTAL_COL: u16 = 1;
const MINUS_GOLD_TOTAL_COL: u16 = 2;
const NO_BARGAINS_TOTAL_COL: u16 = 3;
const PLUS_GOLD_WINS_COL: u16 = 4;
const PLUS_GOLD_LOSSES_COL: u16 = 5;
const PLUS_GOLD_WR_COL: u16 = 6;
const MINUS_GOLD_WINS_COL: u16 = 7;
const MINUS_GOLD_LOSSES_COL: u16 = 8;
const MINUS_GOLD_WR_COL: u16 = 9;
const NO_BARGAINS_WINS_COL: u16 = 10;
const NO_BARGAINS_LOSSES_COL: u16 = 11;
const NO_BARGAINS_WR_COL: u16 = 12;
const MAX_PLUS_GOLD_COL: u16 = 13;
const MAX_MINUS_GOLD_COL: u16 = 14;
const AVG_PLUS_GOLD_COL: u16 = 15;
const AVG_MINUS_GOLD_COL: u16 = 16;


const BARGAINS_TOTAL_STATS_NAMES: [&str; 8] = [
    "Общий средний торг",
    "Суммарно игр с плюсовым торгом",
    "Суммарно игр с минусовым торгом",
    "Суммарно игр без торга",
    "Общий винрейт с плюсовым торгом",
    "Общий винрейт с минусовым торгом",
    "Общий винрейт без торга",
    "Общий винрейт фракции"
];

const TOTAL_AVG_BARGAINS_COL: u16 = 0;
const TOTAL_PLUS_GOLD_GAMES_COL: u16 = 1;
const TOTAL_MINUS_GOLD_GAMES_COL: u16 = 2;
const TOTAL_NO_BARGAINS_GAMES_COL: u16 = 3;
const TOTAL_PLUS_GOLD_WR_COL: u16 = 4;
const TOTAL_MINUS_GOLD_WR_COL: u16 = 5;
const TOTAL_NO_BARGAINS_WR_COL: u16 = 6;
const TOTAL_WR_COL: u16 = 7;

#[derive(Debug, Default)]  
pub struct BargainsGameData {
    pub wins: u32,
    pub losses: u32,
    pub win_bargains: Vec<i64>,
    pub loss_bargains: Vec<i64>
}

#[derive(Debug, Default)]
pub struct RaceBargainsStats {
    pub average_bargains: Vec<f64>,
    pub total_plus_bargain_games: u32,
    pub total_minus_bargain_games: u32,
    pub total_no_bargains_games: u32,
    pub total_plus_bargain_wins: u32,
    pub total_minus_bargain_wins: u32,
    pub total_no_bargains_wins: u32
}

pub enum WinrateType {
    Normal(f64),
    NoGames
}

pub struct RaceStatsBuilder {
    bargains_data: HashMap<i64, RaceBargainsStats>
}

impl RaceStatsBuilder {
    pub fn new() -> Self {
        let races_range = std::ops::Range {start: 0, end: 9};
        RaceStatsBuilder {
            bargains_data: HashMap::from_iter(races_range.filter(|r| *r != 0).map(|r| {
                (r, RaceBargainsStats::default())
            }))
        }
    }

    pub fn build(&mut self, model: &TournamentStatsModel, workbook: &mut Workbook) -> Result<(), crate::error::Error> {
        let tournament = model.tournament.as_ref().ok_or(Error::Other("No tournament provided for generation".to_string()))?;
        for race in &model.races {
            match race.id {
                0 => unreachable!(),
                _=> {
                    let mut row_offset = 0;
                    let worksheet = workbook.add_worksheet().set_name(&race.name)?;
                    if tournament.with_bargains {
                        self.build_bargains_stats(race.id, &model.races, &model.games, worksheet)?;
                        row_offset += 14;
                    }
                    self.build_heroes_stats(race, &model.races, &model.heroes, &model.games, worksheet, row_offset)?; 
                }
            }
        }
        Ok(())
    }

    fn build_bargains_stats(&mut self, race: i64, races_data: &[RaceInfo], games_data: &[GameEntry], worksheet: &mut Worksheet) -> Result<(), crate::error::Error> {
        let games_played_with_minus_gold = games_data.iter()
            .filter(|game| {
                (game.first_player_race == race && game.bargains_amount < 0) ||
                (game.second_player_race == race && game.bargains_amount > 0)  
            })
            .collect_vec();

        let games_played_with_plus_gold = games_data.iter()
            .filter(|game| {
                (game.first_player_race == race && game.bargains_amount > 0) ||
                (game.second_player_race == race && game.bargains_amount < 0) 
            })
            .collect_vec();

        let games_played_without_bargains = games_data.iter()
            .filter(|game| {
                !(game.bargains_amount != 0 || game.first_player_race != race && game.second_player_race != race)
            })
            .filter(|game| (game.first_player_race != game.second_player_race))
            .collect_vec();

        // setup table shape
        worksheet.merge_range(0, 3, 0, 8, "Данные о торгах за фракцию", STYLES.get(&Style::TextBoldCentered)?)?;
        let mut data_column = 0;
        let mut data_row = 1;
        for cell_name in BARGAINS_CELLS_NAMES {
            data_column += 1;
            worksheet.set_column_width(data_column, 20)?;
            worksheet.write_with_format(data_row, data_column, cell_name, STYLES.get(&Style::ThinBorderTextWrap)?)?;
        }

        // this one builds bargains info for all opponents races
        worksheet.set_column_width(0, 20)?;
        for opp_race in races_data.iter().filter(|r| r.id != race && r.id != 0) { 
            data_row += 1;
            worksheet.write_with_format(data_row, 0, &opp_race.name, STYLES.get(&Style::TextBoldCentered)?)?;
            self.build_race_bargains_stats(
                race, 
                opp_race.id, 
                &games_played_with_plus_gold, 
                &games_played_with_minus_gold, 
                &games_played_without_bargains,
                worksheet, 
                data_row)?;
        }

        data_row += 2;
        data_column = 0;

        for cell_name in BARGAINS_TOTAL_STATS_NAMES {
            worksheet.write_with_format(data_row, data_column, cell_name, STYLES.get(&Style::ThinBorderTextWrap)?)?;
            data_column += 1;
        }

            data_row += 1;
    
        // this one shows complete race bargains data
        let race_bargains_total_data = self.bargains_data.get(&race).unwrap();

        // println!("Race bargains data for {race}: {race_bargains_total_data:#?}");

        let total_average_bargain = race_bargains_total_data.average_bargains.iter().sum::<f64>() / 7.0;
        let total_plus_bargain_games = race_bargains_total_data.total_plus_bargain_games;
        let total_minus_bargain_games = race_bargains_total_data.total_minus_bargain_games;
        let total_no_bargains_games = race_bargains_total_data.total_no_bargains_games;
        let total_plus_bargain_wins = race_bargains_total_data.total_plus_bargain_wins;
        let total_minus_bargain_wins = race_bargains_total_data.total_minus_bargain_wins;
        let total_no_bargains_wins = race_bargains_total_data.total_no_bargains_wins;
        let total_plus_bargain_winrate = total_plus_bargain_wins as f64 / total_plus_bargain_games as f64 * 100.0;
        let total_minus_bargain_winrate = total_minus_bargain_wins as f64 / total_minus_bargain_games as f64 * 100.0;
        let total_no_bargains_winrate = total_no_bargains_wins as f64 / total_no_bargains_games as f64 * 100.0;

        let total_winrate = (total_plus_bargain_wins + total_minus_bargain_wins + total_no_bargains_wins) as f64 / (total_minus_bargain_games + total_plus_bargain_games + total_no_bargains_games) as f64 * 100.0;

        worksheet.write_with_format(data_row, TOTAL_AVG_BARGAINS_COL, format!("{total_average_bargain:.2}"), STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(data_row, TOTAL_PLUS_GOLD_GAMES_COL, total_plus_bargain_games, STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(data_row, TOTAL_MINUS_GOLD_GAMES_COL, total_minus_bargain_games, STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(data_row, TOTAL_NO_BARGAINS_GAMES_COL, total_no_bargains_games, STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(data_row, TOTAL_PLUS_GOLD_WR_COL, format!("{total_plus_bargain_winrate:.3}%"), STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(data_row, TOTAL_MINUS_GOLD_WR_COL, format!("{total_minus_bargain_winrate:.3}%"), STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(data_row, TOTAL_NO_BARGAINS_WR_COL, format!("{total_no_bargains_winrate:.3}%"), STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(data_row, TOTAL_WR_COL, format!("{total_winrate:.3}%"), STYLES.get(&Style::ThinBorderTextWrap)?)?;
        
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn build_race_bargains_stats(
        &mut self, 
        race: i64, 
        opp_race: i64, 
        plus_gold_games: &[&GameEntry], 
        minus_gold_games: &[&GameEntry],
        no_bargains_games: &[&GameEntry],
        worksheet: &mut Worksheet, data_row: u32
    ) -> Result<(), crate::error::Error> {
        let mut plus_bargains_data = BargainsGameData { wins: 0, losses: 0, win_bargains: vec![], loss_bargains: vec![] };
        plus_gold_games.iter()
            .filter(|game| game.first_player_race == opp_race || game.second_player_race == opp_race)
            //.unique_by(|game| game.id)
            .for_each(|game| {
                if game.first_player_race == race && game.result == GameResult::FirstPlayerWon || game.second_player_race == race && game.result == GameResult::SecondPlayerWon {
                    plus_bargains_data.wins += 1;
                    plus_bargains_data.win_bargains.push(game.bargains_amount.abs());
                }
                else if game.first_player_race == race && game.result == GameResult::SecondPlayerWon || game.second_player_race == race && game.result == GameResult::FirstPlayerWon {
                    plus_bargains_data.losses += 1;
                    plus_bargains_data.loss_bargains.push(game.bargains_amount.abs());
                }
            });
        
        let plus_games_count = plus_bargains_data.wins + plus_bargains_data.losses;
        
        if let Some(race_bargains_data) = self.bargains_data.get_mut(&race) {
            race_bargains_data.total_plus_bargain_games += plus_games_count;
            race_bargains_data.total_plus_bargain_wins += plus_bargains_data.wins;
        }
        else {
            println!("Problems with fetching bargains data for {}", &race);
        }
        //builder.bargains_data.get_mut(&race.id).unwrap().total_plus_bargain_games += plus_games_count;
        //builder.bargains_data.get_mut(&race.id).unwrap().total_plus_bargain_wins += plus_bargains_data.wins;

        // println!("{:#?}", &plus_bargains_data);
        worksheet.write_with_format(data_row, PLUS_GOLD_TOTAL_COL, plus_games_count, STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(data_row, PLUS_GOLD_WINS_COL, plus_bargains_data.wins, STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(data_row, PLUS_GOLD_LOSSES_COL, plus_bargains_data.losses, STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(
            data_row, 
            PLUS_GOLD_WR_COL, 
        match calc_winrate(plus_bargains_data.wins, plus_games_count) {
                WinrateType::NoGames => {
                    "Не игралось в плюс".to_string()
                },
                WinrateType::Normal(winrate) => {
                    format!("{winrate:.3}%")
                }
            }, 
            STYLES.get(&Style::ThinBorderTextWrap)?
        )?;
        worksheet.write_with_format(
            data_row, 
            MAX_PLUS_GOLD_COL, 
            if let Some(max_plus) = get_max_bargain(&plus_bargains_data.win_bargains, &plus_bargains_data.loss_bargains) {
                format!("{}", &max_plus)
            } else {
                "Не игралось в плюс".to_string()
            }, 
    STYLES.get(&Style::ThinBorderTextWrap)?
        )?;


        let mut minus_bargains_data = BargainsGameData { wins: 0, losses: 0, win_bargains: vec![], loss_bargains: vec![] };
        minus_gold_games.iter()
            .filter(|game| game.first_player_race == opp_race || game.second_player_race == opp_race)
            //.unique_by(|game| game.id)
            .for_each(|game| {
                if game.first_player_race == race && game.result == GameResult::FirstPlayerWon || game.second_player_race == race && game.result == GameResult::SecondPlayerWon {
                    minus_bargains_data.wins += 1;
                    minus_bargains_data.win_bargains.push(if game.bargains_amount >= 0 { -game.bargains_amount } else { game.bargains_amount });
                }
                else if game.first_player_race == race && game.result == GameResult::SecondPlayerWon || game.second_player_race == race && game.result == GameResult::FirstPlayerWon {
                    minus_bargains_data.losses += 1;
                    minus_bargains_data.loss_bargains.push(if game.bargains_amount >= 0 { -game.bargains_amount } else { game.bargains_amount });
                }
            });
        
        let minus_games_count = minus_bargains_data.wins + minus_bargains_data.losses;

        self.bargains_data.get_mut(&race).unwrap().total_minus_bargain_games += minus_games_count;
        self.bargains_data.get_mut(&race).unwrap().total_minus_bargain_wins += minus_bargains_data.wins;

        worksheet.write_with_format(data_row, MINUS_GOLD_TOTAL_COL, minus_games_count, STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(data_row, MINUS_GOLD_WINS_COL, minus_bargains_data.wins, STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(data_row, MINUS_GOLD_LOSSES_COL, minus_bargains_data.losses, STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(
            data_row, 
            MINUS_GOLD_WR_COL, 
            match calc_winrate(minus_bargains_data.wins, minus_games_count) {
                WinrateType::NoGames => {
                    "Не игралось в минус".to_string()
                },
                WinrateType::Normal(winrate) => {
                    format!("{winrate:.3}%")
                }
            },
            STYLES.get(&Style::ThinBorderTextWrap)?
        )?;
        worksheet.write_with_format(
            data_row, 
            MAX_MINUS_GOLD_COL, 
            if let Some(max_minus) = get_min_bargain(&minus_bargains_data.win_bargains, &minus_bargains_data.loss_bargains) {
                format!("{}", &max_minus)
            } else {
                "Не игралось в минус".to_string()
            }, 
    STYLES.get(&Style::ThinBorderTextWrap)?
        )?;

        let mut no_bargains_data = BargainsGameData::default();
        no_bargains_games.iter()
            .filter(|game| game.first_player_race == opp_race || game.second_player_race == opp_race)
            //.unique_by(|game| game.id)
            .for_each(|game| {
                if game.first_player_race == race && game.result == GameResult::FirstPlayerWon || game.second_player_race == race && game.result == GameResult::SecondPlayerWon {
                    no_bargains_data.wins += 1;
                }
                else if game.first_player_race == race && game.result == GameResult::SecondPlayerWon || game.second_player_race == race && game.result == GameResult::FirstPlayerWon {
                    no_bargains_data.losses += 1;
                }
            });

        let no_bargains_games_count = no_bargains_data.wins + no_bargains_data.losses;

        if let Some(race_bargains_data) = self.bargains_data.get_mut(&race) {
            race_bargains_data.total_no_bargains_games += no_bargains_games_count;
            race_bargains_data.total_no_bargains_wins += no_bargains_data.wins;
        }
        else {
            println!("Problems with fetching bargains data for {}", &race);
        }

        worksheet.write_with_format(data_row, NO_BARGAINS_TOTAL_COL, no_bargains_games_count, STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(data_row, NO_BARGAINS_WINS_COL, no_bargains_data.wins, STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(data_row, NO_BARGAINS_LOSSES_COL, no_bargains_data.losses, STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(
            data_row, 
            NO_BARGAINS_WR_COL, 
            match calc_winrate(no_bargains_data.wins, no_bargains_games_count) {
                WinrateType::NoGames => {
                    "Не игралось без торгов".to_string()
                },
                WinrateType::Normal(winrate) => {
                    format!("{winrate:.3}%")
                }
            },
            STYLES.get(&Style::ThinBorderTextWrap)?
        )?;

        // average bargains 
        let plus_bargains_sum = plus_bargains_data.win_bargains.iter().sum::<i64>() + plus_bargains_data.loss_bargains.iter().sum::<i64>();
        let minus_bargains_sum = minus_bargains_data.win_bargains.iter().sum::<i64>() + minus_bargains_data.loss_bargains.iter().sum::<i64>();
        let bargains_sum = plus_bargains_sum + minus_bargains_sum;

        // average in this pair
        self.bargains_data.get_mut(&race).unwrap().average_bargains.push(if bargains_sum == 0 { 0.0 } else { 
            bargains_sum as f64 / (plus_games_count + minus_games_count) as f64
        });

        worksheet.write_with_format(
            data_row, 
            AVG_PLUS_GOLD_COL, 
            if plus_games_count == 0 { "Не игралось в плюс".to_string() } else { format!("{:.3}", plus_bargains_sum as f64 / plus_games_count as f64)}, 
            STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(
            data_row, 
            AVG_MINUS_GOLD_COL, 
            if minus_games_count == 0 { "Не игралось в минус".to_string() } else { format!("{:.3}", minus_bargains_sum as f64 / minus_games_count as f64)},
            STYLES.get(&Style::ThinBorderTextWrap)?)?;
        Ok(())
    }

    fn build_heroes_stats(
        &mut self, 
        race: &RaceInfo, 
        races_data: &[RaceInfo], 
        heroes_data: &Vec<GetHeroesHeroesNewHeroesEntities>, 
        games_data: &Vec<GameEntry>, 
        worksheet: &mut Worksheet,
        row: u32
    ) -> Result<(), Error> {
        // let width = heroes_data.iter()
        //     .filter(|h| h.race == race.id)
        //     .map(|h| h.name.clone())
        //     .collect::<Vec<String>>().iter()
        //     .max_by_key(|x| x.len()).ok_or(crate::error::Error::Other("Max by key error".to_string()))?
        //     .chars()
        //     .count();
        // worksheet.set_column_width(0, (width + 1) as f64)?;
        worksheet.merge_range(row, 4, row, 9, "Общая статистика использования героев", STYLES.get(&Style::TextBoldCentered)?)?;
        let mut row = row + 2;
        let mut heroes_count = 0;

        let mut heroes_used_by_race  = vec![];
    
        let total_race_picks = games_data.iter()
            .filter(|game| {
                game.first_player_race == race.id || game.second_player_race == race.id 
            })
            .collect::<Vec<&GameEntry>>();
    
        total_race_picks.iter()
            .for_each(|game| {
                if game.first_player_race == race.id {
                    if let Some(hero) = heroes_data.iter().find(|hero| hero.id == game.first_player_hero) {
                        heroes_used_by_race.push(hero);
                    }
                    else {
                        //println!("Game with problem: {:?}", game);
                    }
                }
                else if game.second_player_race == race.id {
                    if let Some(hero) = heroes_data.iter().find(|hero| hero.id == game.second_player_hero) {
                        heroes_used_by_race.push(hero);
                    }
                    else {
                        //println!("Game with problem: {:?}", game);
                    }
                    //heroes_used_by_race.push(heroes_data.iter().find(|hero| hero.id == game.second_player_hero).unwrap());
                }
            });
        
        let unique_picked_heroes = heroes_used_by_race.into_iter()
            .unique_by(|hero| hero.id)
            .collect::<Vec<&GetHeroesHeroesNewHeroesEntities>>();
    
        worksheet.write_with_format(row - 1, 1, "Всего побед", STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(row - 1, 2, "Всего поражений", STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(row - 1, 3, "Всего игр", STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(row - 1, 4, "Процент выбора", STYLES.get(&Style::ThinBorderTextWrap)?)?;
    
        let mut col_offset = 5;
        for opp_race in races_data.iter().filter(|r| r.id != 0 && r.id != race.id) {
            worksheet.write_with_format(row - 1, col_offset, format!("Игр vs {}", &opp_race.name), STYLES.get(&Style::ThinBorderTextWrap)?)?;
            worksheet.write_with_format(row - 1, col_offset + 1, format!("Винрейт vs {}", &opp_race.name), STYLES.get(&Style::ThinBorderTextWrap)?)?;
            col_offset += 2;
        }
    
        for hero in &unique_picked_heroes {
            let hero_wins = games_data.iter().filter(|game| {
                (game.first_player_hero == hero.id && game.first_player_race == race.id && game.result == GameResult::FirstPlayerWon) ||
                (game.second_player_hero == hero.id && game.second_player_race == race.id && game.result == GameResult::SecondPlayerWon)
            })
            .collect::<Vec<&GameEntry>>();
            
            let hero_losses = games_data.iter().filter(|game| {
                (game.first_player_hero == hero.id && game.first_player_race == race.id && game.result == GameResult::SecondPlayerWon) ||
                (game.second_player_hero == hero.id && game.second_player_race == race.id && game.result == GameResult::FirstPlayerWon)
            })
            .collect::<Vec<&GameEntry>>();
    
            let total_hero_games = hero_losses.len() + hero_wins.len();
    
            worksheet.write_with_format(row + heroes_count, 0, &hero.name, STYLES.get(&Style::TextBoldCentered)?)?;
            worksheet.write_with_format(row + heroes_count, 1, hero_wins.len() as u32, STYLES.get(&Style::ThinBorderTextWrap)?)?;
            worksheet.write_with_format(row + heroes_count, 2, hero_losses.len() as u32, STYLES.get(&Style::ThinBorderTextWrap)?)?;
            worksheet.write_with_format(row + heroes_count, 3, total_hero_games as u32, STYLES.get(&Style::ThinBorderTextWrap)?)?;
            // pickrate
            worksheet.write_with_format(
                row + heroes_count, 
                4, 
                format!("{:.3}%", 
                total_hero_games as f64 / total_race_picks.len() as f64 * 100.0), 
                STYLES.get(&Style::ThinBorderTextWrap)?
            )?;
    
            let mut col_offset = 5;
            for opp_race in races_data.iter().filter(|r| r.id != 0 && r.id != race.id) {
                let opp_race_wins = total_race_picks.iter()
                    .filter(|game| {
                        (game.first_player_hero == hero.id && game.first_player_race == race.id && game.second_player_race == opp_race.id && game.result == GameResult::FirstPlayerWon) || 
                        (game.second_player_hero == hero.id && game.second_player_race == race.id && game.first_player_race == opp_race.id && game.result == GameResult::SecondPlayerWon) 
                    }).copied()
                    .collect::<Vec<&GameEntry>>()
                    .len();
    
                let opp_race_losses = total_race_picks.iter()
                    .filter(|game| {
                        (game.first_player_hero == hero.id && game.first_player_race == race.id && game.second_player_race == opp_race.id && game.result == GameResult::SecondPlayerWon) || 
                        (game.second_player_hero == hero.id && game.second_player_race == race.id && game.first_player_race == opp_race.id && game.result == GameResult::FirstPlayerWon) 
                    }).copied()
                    .collect::<Vec<&GameEntry>>()
                    .len();
                    
                let total_opp_race_games = opp_race_wins + opp_race_losses;
                worksheet.write_with_format(
                    row + heroes_count, 
                    col_offset, 
                    if total_opp_race_games == 0 { "Нет игр".to_string() } else { total_opp_race_games.to_string() }, 
                    STYLES.get(&Style::ThinBorderTextWrap)?)?;
                worksheet.write_with_format(
                    row + heroes_count, 
                    col_offset + 1, 
                    if total_opp_race_games == 0 { "Нет игр".to_string() } else { format!("{:.3}%", opp_race_wins as f64 / total_opp_race_games as f64 * 100.0) },
                    STYLES.get(&Style::ThinBorderTextWrap)?)?;
                col_offset += 2;
            }
    
            heroes_count += 1;
        }
    
        row += heroes_count + 1;
    
        for opp_race in races_data.iter().filter(|r| r.id != 0 && r.id != race.id) {
            build_hero_stats_vs_race(race, &unique_picked_heroes, heroes_data, opp_race, games_data, worksheet, row)?;
            row += heroes_count + 4;
        }
        Ok(())
    }
}

impl Default for RaceStatsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn build_hero_stats_vs_race(
    race: &RaceInfo, 
    race_heroes: &Vec<&GetHeroesHeroesNewHeroesEntities>, 
    heroes_data: &Vec<GetHeroesHeroesNewHeroesEntities>, 
    opp_race: &RaceInfo, 
    games_data: &Vec<GameEntry>, 
    worksheet: &mut Worksheet, 
    row_offset: u32
) -> Result<(), Error> {
    worksheet.merge_range(
        row_offset, 
        4, 
        row_offset, 
        9, 
        &format!("{} vs {}", race.name, opp_race.name), 
        STYLES.get(&Style::TextBoldCentered)?)?;
    worksheet.merge_range(row_offset + 1, 0, row_offset + 2, 0, "VS", STYLES.get(&Style::TextCenterColorRed)?)?;
    
    let opp_race_heroes = heroes_data.iter()
        .filter(|h| h.race == opp_race.id)
        .collect::<Vec<&GetHeroesHeroesNewHeroesEntities>>();

    let mut col_offset = 1;
    let mut heroes_count = 0;

    for hero in race_heroes {
        worksheet.write_with_format(row_offset + 3 + heroes_count, 0, &hero.name, STYLES.get(&Style::TextBoldCentered)?)?;
        heroes_count += 1;
    }

    for hero in &opp_race_heroes {
        worksheet.merge_range(row_offset + 1, col_offset, row_offset + 1, col_offset + 1, &hero.name, STYLES.get(&Style::TextBoldCentered)?)?;
        worksheet.set_column_width(col_offset, 12)?.set_column_width(col_offset + 1, 12)?;
        worksheet.write_with_format(row_offset + 2, col_offset, "Побед", STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(row_offset + 2, col_offset + 1, "Поражений", STYLES.get(&Style::ThinBorderTextWrap)?)?;
        col_offset += 2;
    }

    // worksheet.set_column_width(col_offset + 1, 12)?.set_column_width(col_offset + 2, 12)?;
    worksheet.write_with_format(row_offset + 1, col_offset + 1, "Всего игр", STYLES.get(&Style::ThinBorderTextWrap)?)?;
    worksheet.write_with_format(row_offset + 1, col_offset + 2, "Винрейт", STYLES.get(&Style::ThinBorderTextWrap)?)?;
    worksheet.set_cell_format(row_offset + 2, col_offset + 1, STYLES.get(&Style::BackgroundSilver)?)?;
    worksheet.set_cell_format(row_offset + 2, col_offset + 2, STYLES.get(&Style::BackgroundSilver)?)?;

    heroes_count = 0;
    for hero in race_heroes {
        let mut opp_hero_count = 1;
        let mut total_games = 0;
        let mut total_wins = 0;
        for opp_hero in &opp_race_heroes {
            let (wins, losses) = get_heroes_pair_stats(hero, race, opp_hero, games_data);
            if wins == 0 {
                worksheet.write_with_format(row_offset + heroes_count + 3, opp_hero_count, wins, STYLES.get(&Style::ThinBorderTextWrap)?)?;
            }
            else {
                worksheet.write_with_format(row_offset + heroes_count + 3, opp_hero_count, wins, STYLES.get(&Style::BackgroundGreen)?)?;
            }
            if losses == 0 {
                worksheet.write_with_format(row_offset + heroes_count + 3, opp_hero_count + 1, losses, STYLES.get(&Style::ThinBorderTextWrap)?)?;
            }
            else {
                worksheet.write_with_format(row_offset + heroes_count + 3, opp_hero_count + 1, losses, STYLES.get(&Style::BackgroundRed)?)?;
            }
            total_games += wins + losses;
            total_wins += wins;
            opp_hero_count += 2;
        }

        worksheet.write_with_format(
            row_offset + heroes_count + 3, 
            opp_hero_count + 1, 
            if total_games == 0 { "Нет игр".to_string() } else { total_games.to_string() }, STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(
            row_offset + heroes_count + 3, 
            opp_hero_count + 2,
            if total_games == 0 { "Нет игр".to_string() } else { format!("{:.3}%", total_wins as f64 / total_games as f64 * 100.0) },
            STYLES.get(&Style::ThinBorderTextWrap)?
        )?;

        heroes_count += 1;
    }

    Ok(())
}

fn get_heroes_pair_stats(hero: &GetHeroesHeroesNewHeroesEntities, race: &RaceInfo, opp_hero: &GetHeroesHeroesNewHeroesEntities, games_data: &Vec<GameEntry>) -> (u32, u32) {
    let wins = games_data.iter().filter(|game| {
            (game.first_player_hero == hero.id && game.first_player_race == race.id && game.second_player_hero == opp_hero.id && game.result == GameResult::FirstPlayerWon) || 
            (game.second_player_hero == hero.id && game.second_player_race == race.id &&  game.first_player_hero == opp_hero.id && game.result == GameResult::SecondPlayerWon)
        })
        .collect::<Vec<&GameEntry>>()
        .len();
    let losses = games_data.iter().filter(|game| {
            (game.first_player_hero == hero.id && game.first_player_race == race.id && game.second_player_hero == opp_hero.id && game.result == GameResult::SecondPlayerWon) || 
            (game.second_player_hero == hero.id && game.second_player_race == race.id && game.first_player_hero == opp_hero.id && game.result == GameResult::FirstPlayerWon)
        })
        .collect::<Vec<&GameEntry>>()
        .len();
    (wins as u32, losses as u32)
}

// self expl
fn get_max_bargain(win_bargains: &[i64], loss_bargains: &[i64]) -> Option<i64> {
    let max_win_bargain = if let Some(max) = win_bargains.iter().max() { *max } else { i64::MIN };
    let max_loss_bargain = if let Some(max) = loss_bargains.iter().max() { *max } else { i64::MIN };

    if max_win_bargain == i64::MIN && max_loss_bargain == i64::MIN {
        return None;
    }    
    Some(max_win_bargain.max(max_loss_bargain))
}

// self expl
fn get_min_bargain(win_bargains: &[i64], loss_bargains: &[i64]) -> Option<i64> {
    let min_win_bargain = if let Some(min) = win_bargains.iter().min() { *min } else { i64::MAX };
    let min_loss_bargain = if let Some(min) = loss_bargains.iter().min() { *min } else { i64::MAX };

    if min_win_bargain == i64::MAX && min_loss_bargain == i64::MAX {
        return None;
    }    
    Some(min_win_bargain.min(min_loss_bargain))
}

fn calc_winrate(wins: u32, total_games: u32) -> WinrateType {
    if total_games == 0 {
        return WinrateType::NoGames;
    }
    WinrateType::Normal((wins as f64 / total_games as f64) * 100.0)
}