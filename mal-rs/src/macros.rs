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