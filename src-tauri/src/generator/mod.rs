use serde::{Deserialize, Serialize};
use types::GameEntry;

use crate::graphql::queries::{get_heroes::GetHeroesHeroesNewHeroesEntities, get_matches::GetMatchesMatches, get_tournament::GetTournamentTournament, get_users::GetUsersUsers};

pub mod commands;
pub mod pair;
pub mod race;
pub mod player;
pub mod styles;
pub mod types;

#[derive(Debug, Serialize, Deserialize)]
pub struct RaceInfo {
    pub id: i64,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TournamentStatsModel {
    pub tournament: Option<GetTournamentTournament>,
    pub users: Vec<GetUsersUsers>,
    pub matches: Vec<GetMatchesMatches>,
    pub games: Vec<GameEntry>,
    pub races: Vec<RaceInfo>,
    pub heroes: Vec<GetHeroesHeroesNewHeroesEntities>
}

impl Default for TournamentStatsModel {
    fn default() -> Self {
        TournamentStatsModel { 
            tournament: None, 
            users: vec![], 
            matches: vec![], 
            games: vec![], 
            races: vec![
                RaceInfo {
                    id: 1,
                    name: "Орден порядка".to_string()
                },
                RaceInfo {
                    id: 2,
                    name: "Инферно".to_string()
                },
                RaceInfo {
                    id: 3,
                    name: "Некрополис".to_string()
                },
                RaceInfo {
                    id: 4,
                    name: "Лесной союз".to_string()
                },
                RaceInfo {
                    id: 5,
                    name: "Лига теней".to_string()
                },
                RaceInfo {
                    id: 6,
                    name: "Академия волшебства".to_string()
                },
                RaceInfo {
                    id: 7,
                    name: "Северные кланы".to_string()
                },
                RaceInfo {
                    id: 8,
                    name: "Великая орда".to_string()
                }
            ], heroes: vec![] }
    }
}