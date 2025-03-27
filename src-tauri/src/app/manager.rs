use tokio::sync::{Mutex, RwLock};

use crate::graphql::queries::{get_games::GetGamesGames, get_heroes::GetHeroesHeroesNewHeroesEntities, get_users::GetUsersUsers};

pub struct AppManager {
    pub current_heroes: RwLock<Vec<GetHeroesHeroesNewHeroesEntities>>,
    pub current_users: RwLock<Vec<GetUsersUsers>>,
    pub current_games: RwLock<Vec<GetGamesGames>>
}