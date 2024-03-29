use dotenvy;
use mal_api::oauth::RedirectResponse;
use mal_api::prelude::*;
use std::io;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let client_id = OauthClient::load_client_id_from_env().unwrap();
    let client_secret = OauthClient::load_client_secret_from_env().unwrap();
    let redirect_url = OauthClient::load_redirect_url_from_env().unwrap();
    let mut oauth_client =
        OauthClient::new(&client_id, Some(&client_secret), &redirect_url).unwrap();
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
    let fields = mal_api::user_fields!(UserField::id, UserField::name, UserField::is_supporter);
    let query = GetUserInformation::new(Some(&fields));
    let response = api_client.get_my_user_information(&query).await.unwrap();
    println!("Information about yourself: {:?}", response);
}
