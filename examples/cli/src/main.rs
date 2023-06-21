use dotenv;
use mal_rs::oauth::{OauthClient, RedirectResponse};
use std::io;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    println!("Hello, world!");
    let mut oauth_client = OauthClient::new();
    println!("URL: {}", oauth_client.generate_readonly_auth_url());

    println!();
    println!("Please enter the URL you were redirected to: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    let response = RedirectResponse::try_from(input).unwrap();

    let token = oauth_client.authenticate(response).await;
    match token {
        Ok(t) => println!("Got token: {:?}", t),
        Err(e) => println!("Failed: {}", e),
    }
}
