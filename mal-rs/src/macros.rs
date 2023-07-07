//! Module for ease-of-use macros

/// Macro for creating a vector of valid AnimeFields
#[cfg(feature = "anime")]
#[macro_export]
macro_rules! anime_fields {
    ($($variant:path),* $(,)?) => {
        {
            let mut v = Vec::new();
            $(
                v.push($variant);
            )*
            AnimeFields(v)
        }
    };
}

/// Macro for creating a vector of valid MangaFields
#[cfg(feature = "manga")]
#[macro_export]
macro_rules! manga_fields {
    ($($variant:path),* $(,)?) => {
        {
            let mut v = Vec::new();
            $(
                v.push($variant);
            )*
            MangaFields(v)
        }
    };
}

/// Macro for creating a vector of valid UserFields
#[cfg(feature = "user")]
#[macro_export]
macro_rules! user_fields {
    ($($variant:path),* $(,)?) => {
        {
            let mut v = Vec::new();
            $(
                v.push($variant);
            )*
            UserFields(v)
        }
    };
}