use dotenv;
use mal_rs::{oauth::RedirectResponse, prelude::*};
use std::io;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let mut oauth_client = OauthClient::new();
    println!("Visit this URL: {}\n", oauth_client.generate_auth_url());

    println!("After authorizing, please enter the URL you were redirected to: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    let response = RedirectResponse::try_from(input).unwrap();

    // Authentication process
    let result = oauth_client.authenticate(response).await;
    let authenticated_oauth_client = match result {
        Ok(t) => {
            println!("Got token: {:?}\n", t.get_access_token_secret());

            let t = t.refresh().await.unwrap();
            println!("Refreshed token: {:?}", t.get_access_token_secret());
            t
        }
        Err(e) => panic!("Failed: {}", e),
    };

    let anime_api_client = AnimeApiClient::from(&authenticated_oauth_client);
    let manga_api_client = MangaApiClient::from(&authenticated_oauth_client);

    // Update One Piece episodes watched
    let query = UpdateMyAnimeListStatus::new(
        21,
        None,
        None,
        None,
        Some(1069),
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();
    let response = anime_api_client.update_anime_list_status(&query).await;
    if let Ok(response) = response {
        println!("Response: {}\n", response);
    }

    let query = UpdateMyMangaListStatus::new(
        91941,
        None,
        None,
        None,
        None,
        Some(57),
        None,
        None,
        None,
        None,
        None,
    )
    .unwrap();
    let response = manga_api_client.update_manga_list_status(&query).await;
    if let Ok(response) = response {
        println!("Response: {}\n", response);
    }
}
