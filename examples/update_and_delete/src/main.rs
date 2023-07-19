use dotenv;
use mal_rs::{oauth::RedirectResponse, prelude::*};
use std::io;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let mut oauth_client = OauthClient::new().unwrap();
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
    let query = UpdateMyAnimeListStatus::builder(21)
        .num_watched_episodes(1069)
        .build()
        .unwrap();
    let response = anime_api_client.update_anime_list_status(&query).await;
    if let Ok(response) = response {
        println!("Response: {}\n", response);
    }

    let query = UpdateMyMangaListStatus::builder(91941)
        .num_chapters_read(57)
        .build()
        .unwrap();
    let response = manga_api_client.update_manga_list_status(&query).await;
    if let Ok(response) = response {
        println!("Response: {}\n", response);
    }

    let query = DeleteMyAnimeListItem::new(52619);
    let response = anime_api_client.delete_anime_list_item(&query).await;
    if let Ok(_) = response {
        println!("Deleted anime entry");
    }

    let query = DeleteMyMangaListItem::new(48881);
    let response = manga_api_client.delete_manga_list_item(&query).await;
    if let Ok(_) = response {
        println!("Deleted manga entry");
    }
}
