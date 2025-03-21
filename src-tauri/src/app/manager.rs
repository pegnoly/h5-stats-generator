use tokio::sync::RwLock;

use crate::graphql::queries::{get_heroes::GetHeroesHeroesNewHeroesEntities, get_users::GetUsersUsers};

pub struct AppManager {
    pub current_heroes: RwLock<Vec<GetHeroesHeroesNewHeroesEntities>>,
    pub current_users: RwLock<Vec<GetUsersUsers>>
}