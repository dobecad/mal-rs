use dotenv;
use mal_rs::oauth::{OauthClient, RedirectResponse};
use std::io;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let mut oauth_client = OauthClient::new();
    println!(
        "Visit this URL: {}\n",
        oauth_client.generate_readonly_auth_url()
    );

    println!("After authorizing, please enter the URL you were redirected to: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    let response = RedirectResponse::try_from(input).unwrap();

    let token = oauth_client.authenticate(response).await;
    match token {
        Ok(t) => {
            println!("Got token: {:?}\n", t.get_access_token());

            let t = t.refresh().await.unwrap();
            println!("Refreshed token: {:?}", t.get_access_token());
        }
        Err(e) => println!("Failed: {}", e),
    }
}
