use dotenvy;
use mal_rs::oauth::MalClientId;
use mal_rs::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let client_id = MalClientId::from_env().unwrap();
    let api_client = AnimeApiClient::from(&client_id);
    let nsfw = false;
    let limit = Some(3);
    let common_fields = mal_rs::anime::all_common_fields();
    let detail_fields = mal_rs::anime::all_detail_fields();

    let query = GetAnimeList::new(
        "One piece".to_string(),
        nsfw,
        Some(&common_fields),
        limit,
        None,
    )
    .unwrap();
    let response = api_client.get_anime_list(&query).await;
    if let Ok(response) = response {
        println!("Received response: {}\n", response);
    }

    let query = GetAnimeDetails::new(9969, Some(&detail_fields));
    let response = api_client.get_anime_details(&query).await;
    if let Ok(response) = response {
        println!("Received response: {}\n", response);
    }

    let query = GetAnimeRanking::new(
        RankingType::ByPopularity,
        nsfw,
        Some(&common_fields),
        limit,
        None,
    )
    .unwrap();
    let response = api_client.get_anime_ranking(&query).await;
    if let Ok(response) = response {
        println!("Received response: {}\n", response);
    }

    let query = GetSeasonalAnime::new(
        2022,
        Season::Summer,
        nsfw,
        Some(&common_fields),
        Some(SeasonalAnimeSort::AnimeScore),
        limit,
        None,
    )
    .unwrap();
    let response = api_client.get_seasonal_anime(&query).await;
    if let Ok(response) = response {
        println!("Received response: {}\n", response);
    }
}
