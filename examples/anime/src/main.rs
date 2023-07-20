use dotenvy;
use mal_rs::oauth::MalClientId;
use mal_rs::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let client_id = MalClientId::from_env().unwrap();
    let api_client = AnimeApiClient::from(&client_id);
    let common_fields = mal_rs::anime::all_common_fields();
    let detail_fields = mal_rs::anime::all_detail_fields();

    // Using the builder pattern for building the query
    let query = GetAnimeList::builder("One Piece")
        .fields(&common_fields)
        .build()
        .unwrap();
    let response = api_client.get_anime_list(&query).await;
    if let Ok(response) = response {
        println!("Received response: {}\n", response);
        for entry in response.data.iter() {
            println!("Anime Title: {}  Anime ID: {}", entry.node.title, entry.node.id);
        }
    }

    let query = GetAnimeDetails::builder(9969)
        .fields(&detail_fields)
        .build()
        .unwrap();
    let response = api_client.get_anime_details(&query).await;
    if let Ok(response) = response {
        println!("Received response: {}\n", response);
    }

    let query = GetAnimeRanking::builder(RankingType::ByPopularity)
        .enable_nsfw()
        .fields(&common_fields)
        .limit(5)
        .build()
        .unwrap();
    let response = api_client.get_anime_ranking(&query).await;
    if let Ok(response) = response {
        println!("Received response: {}\n", response);
    }

    let query = GetSeasonalAnime::builder(2022, Season::Summer)
        .enable_nsfw()
        .fields(&common_fields)
        .sort(SeasonalAnimeSort::AnimeScore)
        .limit(5)
        .build()
        .unwrap();
    let response = api_client.get_seasonal_anime(&query).await;
    if let Ok(response) = response {
        println!("Received response: {}\n", response);
    }
}
