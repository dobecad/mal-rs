use dotenvy;
use mal_rs::oauth::{OauthClient, RedirectResponse};
use std::io;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let authenticated_client = OauthClient::load_from_config();
    match authenticated_client {
        Ok(_) => {
            println!("An existing authorized Oauth client already exists");
            return;
        }
        Err(_) => println!("No existing Oauth client exists\n"),
    }

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

    // Save credentials to config to be re-used later
    let _ = authenticated_oauth_client.save_to_config();
}
