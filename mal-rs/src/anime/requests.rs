// Structs for crafting Anime Endpoint requests
use serde::Deserialize;

pub struct GetAnimeList {
    q: String,
    limit: u8,
    offset: u16,
    fields: AnimeList
}

pub struct AnimeList {

}