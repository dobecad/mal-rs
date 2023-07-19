use dotenvy;
use mal_rs::oauth::RedirectResponse;
use mal_rs::prelude::*;
use std::io;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

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

    // Using Oauth token to interact with User API
    let api_client = UserApiClient::from(&authenticated_oauth_client);
    let fields = mal_rs::user_fields!(UserField::id, UserField::name, UserField::is_supporter);
    let query = GetUserInformation::new(Some(&fields));
    let response = api_client.get_my_user_information(&query).await.unwrap();
    println!("Information about yourself: {:?}", response);
}
