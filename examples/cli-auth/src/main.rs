use dotenvy;
use mal_api::oauth::{OauthClient, RedirectResponse};
use std::io;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // OAuth2 flow with a client_secret
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


    // OAuth2 flow without a client_secret
    let client_id = OauthClient::load_client_id_from_env().unwrap();
    let redirect_url = OauthClient::load_redirect_url_from_env().unwrap();
    let mut oauth_client = OauthClient::new(&client_id, None, &redirect_url).unwrap();
    println!("Visit this URL: {}\n", oauth_client.generate_auth_url());

    println!("After authorizing, please enter the URL you were redirected to: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    let response = RedirectResponse::try_from(input).unwrap();

    // Authentication process
    let result = oauth_client.authenticate(response).await;
    let _ = match result {
        Ok(t) => {
            println!("Got token: {:?}\n", t.get_access_token_secret());

            let t = t.refresh().await.unwrap();
            println!("Refreshed token: {:?}", t.get_access_token_secret());
            t
        }
        Err(e) => panic!("Failed: {}", e),
    };
}
