use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use uuid::Uuid;

use crate::graphql::queries::{
    get_games::{self, GetGamesGames}, get_heroes::{self, GetHeroesHeroesNewHeroesEntities}, get_matches::{self, GetMatchesMatches}, get_tournament::{self, GetTournamentTournament}, get_tournaments::{self, GetTournamentsTournamentsAll}, get_users::{self, GetUsersUsers}, update_game, GetGames, GetHeroes, GetMatches, GetTournament, GetTournaments, GetUsers, UpdateGame
};

use super::{payloads::UpdateGamePayload, types::ModType};

const MAIN_URL: &'static str = "https://h5-tournaments-api-5epg.shuttle.app/";

pub struct TournamentService {
    client: Client,
}

impl TournamentService {
    pub fn new() -> Self {
        TournamentService {
            client: Client::new(),
        }
    }

    pub async fn get_all_tournaments(
        &self,
    ) -> Result<Vec<GetTournamentsTournamentsAll>, crate::error::Error> {
        let query = GetTournaments::build_query(get_tournaments::Variables {});
        let response = self.client.post(MAIN_URL).json(&query).send().await?;
        let result = response
            .json::<Response<get_tournaments::ResponseData>>()
            .await?;
        match result.data {
            Some(data) => Ok(data.tournaments_all),
            None => Err(crate::error::Error::IncorrectData("GetTournaments".to_string())),
        }
    }

    pub async fn get_tournament(
        &self,
        id: Uuid,
    ) -> Result<Option<GetTournamentTournament>, crate::error::Error> {
        let query = GetTournament::build_query(get_tournament::Variables {
            id: Some(id),
            register_channel_id: None,
            reports_channel_id: None,
        });
        let response = self.client.post(MAIN_URL).json(&query).send().await?;
        let result = response
            .json::<Response<get_tournament::ResponseData>>()
            .await?;
        match result.data {
            Some(data) => Ok(data.tournament),
            None => Err(crate::error::Error::IncorrectData("GetTournament".to_string())),
        }
    }

    pub async fn get_users(
        &self,
        tournament_id: Uuid,
    ) -> Result<Vec<GetUsersUsers>, crate::error::Error> {
        let query = GetUsers::build_query(get_users::Variables {
            tournament_id: tournament_id,
        });
        let response = self.client.post(MAIN_URL).json(&query).send().await?;
        let result = response.json::<Response<get_users::ResponseData>>().await?;
        match result.data {
            Some(data) => Ok(data.users),
            None => Err(crate::error::Error::IncorrectData("GetUsers".to_string())),
        }
    }

    pub async fn get_matches(
        &self,
        tournament_id: Uuid,
        player: Option<Uuid>,
    ) -> Result<Vec<GetMatchesMatches>, crate::error::Error> {
        let query = GetMatches::build_query(get_matches::Variables {
            tournament_id: tournament_id,
            user_id: player,
        });
        let response = self.client.post(MAIN_URL).json(&query).send().await?;
        let result = response
            .json::<Response<get_matches::ResponseData>>()
            .await?;
        match result.data {
            Some(data) => Ok(data.matches),
            None => Err(crate::error::Error::IncorrectData("GetMatches".to_string())),
        }
    }

    pub async fn get_games(&self, match_id: Uuid) -> Result<Vec<GetGamesGames>, crate::error::Error> {
        let query = GetGames::build_query(get_games::Variables { match_id: match_id });
        let response = self.client.post(MAIN_URL).json(&query).send().await?;
        let result = response.json::<Response<get_games::ResponseData>>().await?;
        match result.data {
            Some(data) => Ok(data.games),
            None => Err(crate::error::Error::IncorrectData("GetGames".to_string())),
        }
    }

    pub async fn update_game(&self, payload: UpdateGamePayload) -> Result<(), crate::error::Error> {
        let query = UpdateGame::build_query(payload.into());
        let response = self.client.post(MAIN_URL).json(&query).send().await?;
        let result = response
            .json::<Response<update_game::ResponseData>>()
            .await?;
        match result.data {
            Some(_data) => Ok(()),
            None => Err(crate::error::Error::IncorrectData("UpdateGame".to_string())),
        }
    }

    pub async fn get_heroes(&self, mod_type: ModType) -> Result<Vec<GetHeroesHeroesNewHeroesEntities>, crate::error::Error> {
        let query = GetHeroes::build_query(get_heroes::Variables {mod_type: mod_type.into()});
        let response = self.client.post(MAIN_URL).json(&query).send().await?;
        let result = response
            .json::<Response<get_heroes::ResponseData>>()
            .await?;
        match result.data {
            Some(data) => Ok(data.heroes_new.heroes.entities),
            None => Err(crate::error::Error::IncorrectData("GetHeroes".to_string())),
        }
    }
}
