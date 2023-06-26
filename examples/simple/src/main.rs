use std::env;

use dotenv;
use mal_rs::{
    anime::{
        api::{AnimeApi, AnimeApiClient},
        requests::{AnimeFields, GetAnimeList},
        responses::{AnimeFieldsEnum, AnimeList},
    },
    oauth::ClientId,
};

use mal_rs::manga::{
    api::{MangaApi, MangaApiClient},
    requests::{GetMangaList, MangaFields},
    responses::MangaFieldsEnum,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let client_id = ClientId::new(
        env::var("CLIENT_ID").expect("CLIENT_ID environment variable is not defined"),
    );

    // Anime API example
    let api_client = AnimeApiClient::from(&client_id);
    let fields = AnimeFields(vec![
        AnimeFieldsEnum::id,
        AnimeFieldsEnum::num_episodes,
        AnimeFieldsEnum::title,
    ]);
    let query = GetAnimeList::new("one".to_string(), 5, 0, &fields).unwrap();
    let result = api_client.get_anime_list(query).await.unwrap();
    println!("Result: {:?}", &result);
    
    // Example iterating through pages
    let result: Result<AnimeList, _> = api_client.next(&result).await;
    println!("\nNext result: {:?}", &result);
    
    let result: Result<AnimeList, _> = api_client.prev(&result.unwrap()).await;
    println!("\nPrev result: {:?}", &result);

    // Mangag API example
    let api_client = MangaApiClient::from(&client_id);
    let fields = MangaFields(vec![
        MangaFieldsEnum::id,
        MangaFieldsEnum::num_chapters,
        MangaFieldsEnum::title,
    ]);
    let query = GetMangaList::new("one".to_string(), 5, 0, &fields).unwrap();
    let result = api_client.get_manga_list(query).await.unwrap();
    println!("Result: {:?}", result);
}
