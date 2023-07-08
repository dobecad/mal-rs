use dotenv;
use mal_rs::oauth::MalClientId;
use mal_rs::prelude::*;
use mal_rs::{anime_fields, manga_fields};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let client_id = MalClientId::from_env().unwrap();

    // Anime API example
    let api_client = AnimeApiClient::from(&client_id);
    let fields = anime_fields!(
        AnimeFieldsEnum::id,
        AnimeFieldsEnum::num_episodes,
        AnimeFieldsEnum::title,
    );
    let query = GetAnimeList::new("one".to_string(), Some(5), None, Some(&fields)).unwrap();
    let result = api_client.get_anime_list(&query).await.unwrap();
    println!("Result: {}", &result);

    // Example iterating through pages
    let result: AnimeList = api_client.next(&result).await.unwrap();
    println!("\nNext result: {}", &result);

    let result: AnimeList = api_client.prev(&result).await.unwrap();
    println!("\nPrev result: {}", &result);

    // Mangag API example
    let api_client = MangaApiClient::from(&client_id);
    let fields = mal_rs::manga::all_fields();
    let query = GetMangaList::new("one".to_string(), Some(5), None, Some(&fields)).unwrap();
    let result = api_client.get_manga_list(&query).await.unwrap();
    println!("Result: {}", result);
}
