//! Module for ease-of-use macros

/// Macro for creating a vector of valid AnimeCommonFields
#[cfg(feature = "anime")]
#[macro_export]
macro_rules! anime_common_fields {
    ($($variant:path),* $(,)?) => {
        {
            let mut v = Vec::new();
            $(
                v.push($variant);
            )*
            AnimeCommonFields(v)
        }
    };
}

/// Macro for creating a vector of valid AnimeDetailFields
#[cfg(feature = "anime")]
#[macro_export]
macro_rules! anime_detail_fields {
    ($($variant:path),* $(,)?) => {
        {
            let mut v = Vec::new();
            $(
                v.push($variant);
            )*
            AnimeDetailFields(v)
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