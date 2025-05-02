use std::collections::HashMap;

use itertools::Itertools;
use rust_xlsxwriter::{workbook::Workbook, worksheet::Worksheet};

use crate::{error::Error, graphql::queries::get_heroes::GetHeroesHeroesNewHeroesEntities, services::tournament::types::GameResult};

use super::{styles::{Style, STYLES}, types::GameEntry, RaceInfo, TournamentStatsModel};

#[derive(Debug, Default)]
pub struct RaceBargainsStats {
    pub average_bargains: Vec<f64>,
    pub total_plus_bargain_games: u32,
    pub total_minus_bargain_games: u32,
    pub total_plus_bargain_wins: u32,
    pub total_minus_bargain_wins: u32,
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
        let mut row_offset = 0;
        for race in &model.races {
            match race.id {
                0 => unreachable!(),
                _=> {
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

    fn build_bargains_stats(&mut self, race: i64, races_data: &Vec<RaceInfo>, games_data: &Vec<GameEntry>, worksheet: &mut Worksheet) -> Result<(), crate::error::Error> {
        Ok(())
    }

    fn build_heroes_stats(
        &mut self, 
        race: &RaceInfo, 
        races_data: &Vec<RaceInfo>, 
        heroes_data: &Vec<GetHeroesHeroesNewHeroesEntities>, 
        games_data: &Vec<GameEntry>, 
        worksheet: &mut Worksheet,
        row: u32
    ) -> Result<(), Error> {
        let width = heroes_data.iter()
            .filter(|h| h.race == race.id)
            .map(|h| h.name.clone())
            .collect::<Vec<String>>().iter()
            .max_by_key(|x| x.len()).ok_or(crate::error::Error::Other("Max by key error".to_string()))?
            .chars()
            .count();
        worksheet.set_column_width(0, (width + 1) as f64)?;
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
                    })
                    .map(|game| *game )
                    .collect::<Vec<&GameEntry>>()
                    .len();
    
                let opp_race_losses = total_race_picks.iter()
                    .filter(|game| {
                        (game.first_player_hero == hero.id && game.first_player_race == race.id && game.second_player_race == opp_race.id && game.result == GameResult::SecondPlayerWon) || 
                        (game.second_player_hero == hero.id && game.second_player_race == race.id && game.first_player_race == opp_race.id && game.result == GameResult::FirstPlayerWon) 
                    })
                    .map(|game| *game )
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

    worksheet.set_column_width(col_offset + 1, 12)?.set_column_width(col_offset + 2, 12)?;
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