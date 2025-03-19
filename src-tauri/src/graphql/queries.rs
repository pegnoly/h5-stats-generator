use graphql_client::GraphQLQuery;
use uuid::Uuid;

type UUID = Uuid;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/get_games.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct GetGames;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/get_matches.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct GetMatches;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/get_users.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct GetUsers;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/update_game.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct UpdateGame;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/get_tournaments.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct GetTournaments;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/get_tournament.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct GetTournament;
