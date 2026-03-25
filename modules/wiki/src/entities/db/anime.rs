/// Anime media format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "anime_type", rename_all = "lowercase")]
pub enum AnimeType {
    #[sqlx(rename = "tv")]
    TvSeries,
    Ova,
    #[sqlx(rename = "mov")]
    Movie,
    #[sqlx(rename = "oth")]
    Other,
    Web,
    #[sqlx(rename = "spe")]
    Special,
    #[sqlx(rename = "mv")]
    MusicVideo,
}

/// Anime information fetched from AniDB, used only for cross-referencing with VNs.
///
/// `id` is the AniDB identifier.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Anime {
    pub id: i32,
    pub anime_type: Option<AnimeType>,
    pub year: Option<i16>,
    pub title_romaji: Option<String>,
    pub title_kanji: Option<String>,
}

/// Anime News Network identifier for an anime entry.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct AnimeAnnId {
    pub anime_id: i32,
    pub ann_id: i32,
}

/// MyAnimeList identifier for an anime entry.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct AnimeMalId {
    pub anime_id: i32,
    pub mal_id: i32,
}
