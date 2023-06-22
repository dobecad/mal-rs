use std::env;

use dotenv;
use mal_rs::oauth::ClientId;
use mal_rs::anime::api::AnimeApi;
use mal_rs::anime::requests::GetAnimeList;
use mal_rs::anime::requests::AnimeFields;
use mal_rs::anime::responses::AnimeFieldsEnum;
use mal_rs::anime::api::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let client_id = ClientId::new(env::var("CLIENT_ID").expect("CLIENT_ID environment variable is not defined"));
    let api_client = AnimeApi::from(client_id);

    let fields = AnimeFields(vec![AnimeFieldsEnum::id, AnimeFieldsEnum::num_episodes, AnimeFieldsEnum::title, AnimeFieldsEnum::synopsis]);
    let query = GetAnimeList::new("one".to_string(), 5, 0, fields).unwrap();
    let result = api_client.get_anime_list(query).await.unwrap();
    println!("Result: {:?}", result);

}
