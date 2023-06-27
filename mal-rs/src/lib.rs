pub mod anime;
pub mod common;
pub mod macros;
pub mod manga;
pub mod oauth;
pub mod user;

const OAUTH_URL: &'static str = "https://myanimelist.net/v1/oauth2/authorize";
const OAUTH_TOKEN_URL: &'static str = "https://myanimelist.net/v1/oauth2/token";
const ANIME_URL: &'static str = "https://api.myanimelist.net/v2/anime";
const MANGA_URL: &'static str = "https://api.myanimelist.net/v2/manga";
const USER_URL: &'static str = "https://api.myanimelist.net/v2/users";

pub mod prelude {
    pub use crate::anime::api::*;
    pub use crate::anime::requests::*;
    pub use crate::anime::responses::*;
    pub use crate::manga::api::*;
    pub use crate::manga::requests::*;
    pub use crate::manga::responses::*;

    pub use oauth2::{AccessToken, ClientId};
}

#[cfg(test)]
mod tests {
    use super::*;
}
