use dotenvy;
use mal_rs::oauth::MalClientId;
use mal_rs::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let client_id = MalClientId::from_env().unwrap();
    let api_client = MangaApiClient::from(&client_id);
    let nsfw = false;
    let limit = Some(3);
    let common_fields = mal_rs::manga::all_common_fields();
    let detail_fields = mal_rs::manga::all_detail_fields();

    let query =
        GetMangaList::new("one".to_string(), nsfw, Some(&common_fields), limit, None).unwrap();
    let response = api_client.get_manga_list(&query).await;
    if let Ok(response) = response {
        println!("Response: {}\n", response);
    }

    let query = GetMangaDetails::new(44347, nsfw, Some(&detail_fields)).unwrap();
    let response = api_client.get_manga_details(&query).await;
    if let Ok(response) = response {
        println!("Response: {}\n", response);
    }

    let query = GetMangaRanking::new(
        MangaRankingType::ByPopularity,
        nsfw,
        Some(&common_fields),
        limit,
        None,
    )
    .unwrap();
    let response = api_client.get_manga_ranking(&query).await;
    if let Ok(response) = response {
        println!("Response: {}\n", response);
    }
}
