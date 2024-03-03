use dotenvy;
use mal_api::{
    oauth::{Authenticated, OauthClient, RedirectResponse},
    prelude::{
        AnimeApiClient, GetSuggestedAnime, GetUserAnimeList, GetUserInformation, GetUserMangaList,
        MangaApi, MangaApiClient, UserApiClient,
    },
};
use std::io;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let authenticated_client = OauthClient::load_from_config();
    match authenticated_client {
        Ok(c) => {
            println!("An existing authorized Oauth client already exists");
            endpoints(&c).await;
            return;
        }
        Err(_) => println!("No existing Oauth client exists"),
    }

    let client_id = OauthClient::load_client_id_from_env().unwrap();
    let client_secret = OauthClient::load_client_secret_from_env().unwrap();
    let redirect_url = OauthClient::load_redirect_url_from_env().unwrap();
    let mut oauth_client = OauthClient::new(&client_id, Some(&client_secret), &redirect_url).unwrap();
    println!("Visit this URL: {}\n", oauth_client.generate_auth_url());

    println!("After authorizing, please enter the URL you were redirected to: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    let response = RedirectResponse::try_from(input).unwrap();

    // Authentication process
    let result = oauth_client.authenticate(response).await;
    let authenticated_client = match result {
        Ok(t) => {
            println!("Got token: {:?}\n", t.get_access_token_secret());

            let t = t.refresh().await.unwrap();
            println!("Refreshed token: {:?}", t.get_access_token_secret());
            t
        }
        Err(e) => panic!("Failed: {}", e),
    };

    // Save credentials to config to be re-used later
    let _ = authenticated_client.save_to_config();
}

async fn endpoints(oauth_client: &OauthClient<Authenticated>) {
    let anime_api_client = AnimeApiClient::from(oauth_client);
    let manga_api_client = MangaApiClient::from(oauth_client);
    let user_api_client = UserApiClient::from(oauth_client);

    let query = GetSuggestedAnime::builder()
        .fields(&mal_api::anime::all_common_fields())
        .limit(5)
        .build();
    let response = anime_api_client.get_suggested_anime(&query).await;
    if let Ok(response) = response {
        println!("Response: {}\n", response);
    }

    let query = GetUserAnimeList::builder("@me").limit(5).build().unwrap();
    let response = anime_api_client.get_user_anime_list(&query).await;
    if let Ok(response) = response {
        println!("Response: {}\n", response);
    }

    let query = GetUserMangaList::builder("@me").limit(5).build().unwrap();
    let response = manga_api_client.get_user_manga_list(&query).await;
    if let Ok(response) = response {
        println!("Response: {}\n", response);
    }

    let user_fields = mal_api::user::all_fields();
    let query = GetUserInformation::new(Some(&user_fields));
    let response = user_api_client.get_my_user_information(&query).await;
    if let Ok(response) = response {
        println!("Response: {}\n", response);
    }
}
