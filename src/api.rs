use dotenv_codegen::dotenv;
use eyre::Result;
use graphql_client::{reqwest::post_graphql, GraphQLQuery};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries.graphql",
    response_derives = "Debug"
)]
pub struct CreateGameMutation;

pub async fn create_game(name: &str) -> Result<()> {
    let graphql_uri = dotenv!("GRAPHQL_URI");
    let variables = create_game_mutation::Variables {
        name: Some(name.into()),
    };
    let client = reqwest::Client::new();
    let result = post_graphql::<CreateGameMutation, _>(&client, graphql_uri, variables).await?;

    dbg!(result);

    Ok(())
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/queries.graphql",
    response_derives = "Debug, Serialize"
)]
pub struct ListGames;

pub async fn list_games() -> Result<list_games::ResponseData> {
    let graphql_uri = dotenv!("GRAPHQL_URI");
    let variables = list_games::Variables {};
    let client = reqwest::Client::new();
    let response = post_graphql::<ListGames, _>(&client, graphql_uri, variables).await?;

    dbg!(&response);

    Ok(response.data.unwrap())
}
