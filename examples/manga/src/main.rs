use dotenvy;
use mal_api::oauth::MalClientId;
use mal_api::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let client_id = MalClientId::from_env().unwrap();
    let api_client = MangaApiClient::from(&client_id);
    let common_fields = mal_api::manga::all_common_fields();
    let detail_fields = mal_api::manga::all_detail_fields();

    let query = GetMangaList::builder("one")
        .fields(&common_fields)
        .limit(3)
        .build()
        .unwrap();
    let response = api_client.get_manga_list(&query).await;
    if let Ok(response) = response {
        println!("Response: {}\n", response);
    }

    let query = GetMangaDetails::builder(44347)
        .fields(&detail_fields)
        .build()
        .unwrap();
    let response = api_client.get_manga_details(&query).await;
    if let Ok(response) = response {
        println!("Response: {}\n", response);
    }

    let query = GetMangaRanking::builder(MangaRankingType::All)
        .enable_nsfw()
        .fields(&common_fields)
        .limit(10)
        .build()
        .unwrap();
    let response = api_client.get_manga_ranking(&query).await;
    if let Ok(response) = response {
        println!("Response: {}\n", response);
    }
}
