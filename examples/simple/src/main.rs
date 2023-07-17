use dotenv;
use mal_rs::anime_common_fields;
use mal_rs::oauth::MalClientId;
use mal_rs::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let client_id = MalClientId::from_env().unwrap();

    // Anime API example
    let api_client = AnimeApiClient::from(&client_id);
    let nsfw = false;
    let limit = Some(5);
    let fields = anime_common_fields!(
        AnimeField::id,
        AnimeField::num_episodes,
        AnimeField::title,
    );

    let query = GetAnimeList::new("one".to_string(), nsfw, Some(&fields), limit, None).unwrap();
    let result = api_client.get_anime_list(&query).await.unwrap();
    println!("Result: {}", &result);

    // Example iterating through pages
    let result: AnimeList = api_client.next(&result).await.unwrap();
    println!("\nNext result: {}", &result);

    let result: AnimeList = api_client.prev(&result).await.unwrap();
    println!("\nPrev result: {}", &result);

    // Manga API example
    let api_client = MangaApiClient::from(&client_id);
    let fields = mal_rs::manga::all_common_fields();
    let query = GetMangaList::new("one".to_string(), nsfw, Some(&fields), limit, None).unwrap();
    let result = api_client.get_manga_list(&query).await.unwrap();
    println!("Result: {}", result);
}
