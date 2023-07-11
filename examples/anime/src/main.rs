use dotenvy;
use mal_rs::anime_fields;
use mal_rs::oauth::MalClientId;
use mal_rs::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let client_id = MalClientId::from_env().unwrap();
    let api_client = AnimeApiClient::from(&client_id);
    let nsfw = false;
    let limit = Some(3);
    let fields = mal_rs::anime::all_fields();

    let query = GetAnimeList::new("One piece".to_string(), nsfw, Some(&fields), limit, None).unwrap();
    let response = api_client.get_anime_list(&query).await;
    if let Ok(response) = response {
        println!("Received response: {}\n", response);
    }

    let query = GetAnimeDetails::new(459, Some(&fields));
    let response = api_client.get_anime_details(&query).await;
    println!("Response: {:?}", response);
    // if let Ok(response) = response {
    //     println!("Received response: {}", response);
    // }
}
