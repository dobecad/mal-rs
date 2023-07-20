# mal-rs

An asynchronous [MyAnimeList](https://myanimelist.net/) (MAL) API library for Rust.

## Features

- Type-safe query builder
- Type-safe responses
- Pagination through responses
- Support for accessing all of MAL's endpoints (Anime, Manga, Forum, and User)
  - To access the Forum and User endpoints, enable the `forum` and `user` features
- OAuth2 access token retrieval and management

## Example

```rust
use dotenvy;
use mal_rs::anime_common_fields;
use mal_rs::oauth::MalClientId;
use mal_rs::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let client_id = MalClientId::from_env().unwrap();

    let api_client = AnimeApiClient::from(&client_id);
    let fields = anime_common_fields!(
        AnimeField::id, 
        AnimeField::num_episodes, 
        AnimeField::title,
    );

    // Example using builder pattern. The `builder(args...)` method will only require
    // the required arguments for the specific API endpoint, while the
    // other builder instance methods will build up the optional arguments
    let query = GetAnimeList::builder("One")
        .fields(&fields)
        .limit(5)
        .build()
        .unwrap();
    let result = api_client.get_anime_list(&query).await.unwrap();
    println!("Received response: {}", result);
    for entry in result.data.iter() {
        println!("Anime Title: {}  Anime ID: {}", entry.node.title, entry.node.id);
    }

    // Example iterating through pages
    let result = api_client.next(&result).await.unwrap();
    println!("Next result: {}", &result);

    let result = api_client.prev(&result).await.unwrap();
    println!("Prev result: {}", &result);

    // Manga API example
    let api_client = MangaApiClient::from(&client_id);
    let fields = mal_rs::manga::all_common_fields();

    // Example using `new` pattern. Not recommended, but available
    let nsfw = false;
    let limit = Some(5);
    let query = GetMangaList::new("one".to_string(), nsfw, Some(&fields), limit, None).unwrap();
    let result = api_client.get_manga_list(&query).await.unwrap();
    println!("Result: {}", result);
}
```

You can check out the [examples](./examples/) directory to see additional usage examples.

## License

This project is licensed under the [MIT license](./README.md)
