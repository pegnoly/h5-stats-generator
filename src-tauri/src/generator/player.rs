use std::collections::HashMap;

use rust_xlsxwriter::{workbook::Workbook, worksheet::Worksheet};
use uuid::Uuid;
use super::{types::{GameHistoryEntry, PlayerMatchHistoryHeaders}, TournamentStatsModel};
use crate::{error::Error as Error, generator::{styles::{Style, STYLES}, types::{GameEntry, ResultOutput}}, graphql::queries::get_matches::GetMatchesMatches, services::tournament::types::{BargainsColor, GameResult}};

pub fn build_player_stats(model: &TournamentStatsModel, workbook: &mut Workbook) -> Result<(), Error> {
    let tournament = model.tournament.as_ref().ok_or(Error::Other("No tournament provided for generation".to_string()))?;
    let headers_data = PlayerMatchHistoryHeaders::new(&tournament);
    for user in &model.users {
        println!("Generating data for {}", &user.nickname);
        let worksheet = workbook.add_worksheet().set_name(user.nickname.clone())?;
        headers_data.to_xlsx(worksheet)?;
        build_game_history(model, worksheet, user.id)?;
        println!("This user finished");
    }
    Ok(())
}

pub fn build_game_history(model: &TournamentStatsModel, worksheet: &mut Worksheet, user: Uuid) -> Result<(), Error> {

    let mut user_race_games: HashMap<i64, u32> = HashMap::new();
    let mut user_race_wins: HashMap<i64, u32> = HashMap::new();
    let mut user_hero_games: HashMap<i64, u32> = HashMap::new();
    let mut user_hero_wins: HashMap<i64, u32> = HashMap::new(); 

    let user_matches = model.matches.iter()
        .filter(|m| {
            m.first_player == user || m.second_player == user
        })
        .collect::<Vec<&GetMatchesMatches>>();

    println!("Matches for user: {}", &user_matches.len());

    let mut game_row = 2;

    for user_match in user_matches {
        let is_first_player = user_match.first_player == user;
        let opponent = if is_first_player { 
            &model.users.iter()
                .find(|user| user.id == user_match.second_player)
                .ok_or(Error::Other(format!("No user found with id {}", user_match.second_player)))?
                .nickname
        } else {
            &model.users.iter()
            .find(|user| user.id == user_match.first_player)
            .ok_or(Error::Other(format!("No user found with id {}", user_match.first_player)))?
            .nickname
        };

        let games = model.games.iter()
            .filter(|game| {
                game.match_id == user_match.id
            })
            .collect::<Vec<&GameEntry>>();
        
        println!("Games for match with {}: {}", opponent, &games.len());

        for game in games {
            let player_race = if is_first_player {
                model.races.iter()
                    .find(|r| r.id == game.first_player_race)
                    .ok_or(Error::Other("No matching race found".to_string()))?
            } else {
                model.races.iter()
                    .find(|r| r.id == game.second_player_race)
                    .ok_or(Error::Other("No matching race found".to_string()))?
            };

            if let Some(race_games_count) = user_race_games.get_mut(&player_race.id) {
                *race_games_count += 1;
            } else {
                user_race_games.insert(player_race.id, 1);
            }

            let player_hero = if is_first_player {
                model.heroes.iter()
                    .find(|hero| hero.id == game.first_player_hero)
                    .ok_or(Error::Other("No matching hero found".to_string()))?
            } else {
                model.heroes.iter()
                .find(|hero| hero.id == game.second_player_hero)
                .ok_or(Error::Other("No matching hero found".to_string()))?
            };

            if let Some(hero_games_count) = user_hero_games.get_mut(&player_hero.id) {
                *hero_games_count += 1;
            } else {
                user_hero_games.insert(player_hero.id, 1);
            }

            let opponent_race = if is_first_player {
                &model.races.iter()
                    .find(|r| r.id == game.second_player_race)
                    .ok_or(Error::Other("No matching race found".to_string()))?
                    .name
            } else {
                &model.races.iter()
                    .find(|r| r.id == game.first_player_race)
                    .ok_or(Error::Other("No matching race found".to_string()))?
                    .name
            };
            let opponent_hero = if is_first_player {
                &model.heroes.iter()
                    .find(|hero| hero.id == game.second_player_hero)
                    .ok_or(Error::Other("No matching hero found".to_string()))?
                    .name
            } else {
                &model.heroes.iter()
                .find(|hero| hero.id == game.first_player_hero)
                .ok_or(Error::Other("No matching hero found".to_string()))?
                .name
            };

            let bargains_amount = if game.bargains_amount == -1 { 
                None
            } else if !is_first_player {
                Some(-game.bargains_amount)
            } else {
                Some(game.bargains_amount)
            };

            let result = if is_first_player {
                match game.result {
                    GameResult::FirstPlayerWon => {

                        if let Some(race_wins_count) = user_race_wins.get_mut(&player_race.id) {
                            *race_wins_count += 1;
                        } else {
                            user_race_wins.insert(player_race.id, 1);
                        }

                        if let Some(hero_wins_count) = user_hero_wins.get_mut(&player_hero.id) {
                            *hero_wins_count += 1;
                        } else {
                            user_hero_wins.insert(player_hero.id, 1);
                        }

                        ResultOutput::Win
                    },
                    GameResult::SecondPlayerWon => ResultOutput::Loss,
                    _=> unreachable!()
                }
            } else {
                match game.result {
                    GameResult::FirstPlayerWon => ResultOutput::Loss,
                    GameResult::SecondPlayerWon => { 
                        if let Some(race_wins_count) = user_race_wins.get_mut(&player_race.id) {
                            *race_wins_count += 1;
                        } else {
                            user_race_wins.insert(player_race.id, 1);
                        }

                        if let Some(hero_wins_count) = user_hero_wins.get_mut(&player_hero.id) {
                            *hero_wins_count += 1;
                        } else {
                            user_hero_wins.insert(player_hero.id, 1);
                        }
                        ResultOutput::Win
                    },
                    _=> unreachable!()
                }
            };

            println!("Game: {game:#?}");

            let game_history_entry = GameHistoryEntry {
                opponent,
                player_race: &player_race.name,
                player_hero: &player_hero.name,
                opponent_race,
                opponent_hero,
                bargains_amount,
                bargains_color: if let Some(color) = &game.bargains_color {
                    match color {
                        BargainsColor::BargainsColorBlue => Some("Синий"),
                        BargainsColor::BargainsColorRed => Some("Красный"),
                        _=> unreachable!()
                    }
                } else {
                    None
                },
                result,
                outcome: None
            };

            //println!("Game converted: {:#?}", &game_history_entry);

            game_history_entry.to_xlsx(worksheet, game_row)?;
            game_row+=1;
        } 
    }

    let total_winrate_row = game_row + 1;

    let total_user_games = user_race_games.iter()
        .map(|g| {
            *g.1
        })
        .sum::<u32>();

    let total_user_wins = user_race_wins.iter()
        .map(|g| {
            *g.1
        })
        .sum::<u32>();

    worksheet.write_with_format(total_winrate_row, 0, "Всего игр", STYLES.get(&Style::ThinBorderTextWrap)?)?;
    worksheet.write_with_format(total_winrate_row, 1, total_user_games, STYLES.get(&Style::ThinBorderTextWrap)?)?;
    worksheet.write_with_format(total_winrate_row + 1, 0, "Общий винрейт", STYLES.get(&Style::ThinBorderTextWrap)?)?;
    worksheet.write_with_format(
        total_winrate_row + 1, 
        1, 
        format!("{:.3}%", 
        total_user_wins as f64 / total_user_games as f64 * 100.0), STYLES.get(&Style::ThinBorderTextWrap)?
    )?;

    let race_selection_row = total_winrate_row + 4;
    worksheet.merge_range(race_selection_row - 1, 0, race_selection_row - 1, 2, "Выбор рас", STYLES.get(&Style::TextBoldCentered)?)?;
    worksheet.write_with_format(race_selection_row, 1, "Всего игр", STYLES.get(&Style::ThinBorderTextWrap)?)?;
    worksheet.write_with_format(race_selection_row, 2, "Винрейт", STYLES.get(&Style::ThinBorderTextWrap)?)?;

    let mut races_count = 0;
    for race_info in user_race_games {
        races_count += 1;
        let winrate = *user_race_wins.get(&race_info.0).unwrap_or(&0) as f64 / race_info.1 as f64 * 100.0;
        worksheet.write_with_format(
            race_selection_row + races_count, 
            0, 
            &model.races.iter().find(|r| r.id == race_info.0).unwrap().name,
            STYLES.get(&Style::ThinBorderTextWrap)?
        )?;
        worksheet.write_with_format(race_selection_row + races_count, 1, race_info.1, STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(race_selection_row + races_count, 2, format!("{:.3}%", winrate), STYLES.get(&Style::ThinBorderTextWrap)?)?;
    }

    let hero_selection_row = race_selection_row + races_count + 3;
    worksheet.merge_range(hero_selection_row - 1, 0, hero_selection_row - 1, 2, "Выбор героев", STYLES.get(&Style::TextBoldCentered)?)?;
    worksheet.write_with_format(hero_selection_row, 1, "Всего игр", STYLES.get(&Style::ThinBorderTextWrap)?)?;
    worksheet.write_with_format(hero_selection_row, 2, "Винрейт", STYLES.get(&Style::ThinBorderTextWrap)?)?;

    let mut heroes_count = 0;
    for hero_info in user_hero_games {
        heroes_count += 1;
        let winrate = *user_hero_wins.get(&hero_info.0).unwrap_or(&0) as f64 / hero_info.1 as f64 * 100.0;
        worksheet.write_with_format(
            hero_selection_row + heroes_count, 
            0, 
            &model.heroes.iter().find(|h| h.id == hero_info.0).unwrap().name,
            STYLES.get(&Style::ThinBorderTextWrap)?
        )?;
        worksheet.write_with_format(hero_selection_row + heroes_count, 1, hero_info.1, STYLES.get(&Style::ThinBorderTextWrap)?)?;
        worksheet.write_with_format(hero_selection_row + heroes_count, 2, format!("{:.3}%", winrate), STYLES.get(&Style::ThinBorderTextWrap)?)?;
    }

    Ok(())
}