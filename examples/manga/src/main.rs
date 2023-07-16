use dotenvy;
use mal_rs::oauth::MalClientId;
use mal_rs::prelude::*;

#[tokio::main]
async fn main() {
    let client_id = MalClientId::from_env().unwrap();
    let api_client = MangaApiClient::from(&client_id);
    let nsfw = false;
    let limit = Some(3);
}
