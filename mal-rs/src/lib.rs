pub mod anime;
pub mod manga;
pub mod oauth;

const OAUTH_URL: &'static str = "https://myanimelist.net/v1/oauth2/authorize";
const OAUTH_TOKEN_URL: &'static str = "https://myanimelist.net/v1/oauth2/token";
const API_URL: &'static str = "https://api.myanimelist.net/v2";

#[cfg(test)]
mod tests {
    use super::*;
}
