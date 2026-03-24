/// Text-valued Wikidata property discriminator (P-numbers in comments).
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "wikidata_text_prop", rename_all = "snake_case")]
pub enum WikidataTextProp {
    Website,         // P856
    Vndb,            // P3180
    Mobygames,       // P1933
    MobygamesCompany, // P4773
    IndiedbGame,     // P6717
    Crunchyroll,     // P4110
    IgdbGame,        // P5794
    Giantbomb,       // P5247
    Pcgamingwiki,    // P6337
    Gog,             // P2725
    Soundcloud,      // P3040
    Humblestore,     // P4477
    Itchio,          // P7294
    PlaystationJp,   // P5999
    PlaystationNa,   // P5944
    PlaystationEu,   // P5971
    Lutris,          // P7597
    Twitter,         // P2002
    Enwiki,
    Jawiki,
}

/// Integer-valued Wikidata property discriminator (P-numbers in comments).
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "wikidata_int_prop", rename_all = "snake_case")]
pub enum WikidataIntProp {
    GamefaqsGame,     // P4769
    GamefaqsCompany,  // P6182
    AnidbAnime,       // P5646
    AnidbPerson,      // P5649
    AnnAnime,         // P1985
    AnnManga,         // P1984
    VgmdbProduct,     // P5659
    VgmdbArtist,      // P3435
    DiscogsArtist,    // P1953
    AcdbChar,         // P7013
    AcdbSource,       // P7017
    Howlongtobeat,    // P2816
    Steam,            // P1733
    PixivUser,        // P5435
    DoujinshiAuthor,  // P7511
    Wine,             // P600
    MobygamesGame,    // P11688
}

/// UUID-valued Wikidata property discriminator (P-numbers in comments).
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "wikidata_uuid_prop", rename_all = "snake_case")]
pub enum WikidataUuidProp {
    MusicbrainzArtist, // P434
}

/// A Wikidata entity, identified by its Q-number.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Wikidata {
    pub id: i32,
}

/// A text-valued external identifier from Wikidata.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct WikidataTextLink {
    pub wikidata_id: i32,
    pub property: WikidataTextProp,
    pub value: String,
}

/// An integer-valued external identifier from Wikidata.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct WikidataIntLink {
    pub wikidata_id: i32,
    pub property: WikidataIntProp,
    pub value: i32,
}

/// A UUID-valued external identifier from Wikidata (e.g. MusicBrainz artist).
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct WikidataUuidLink {
    pub wikidata_id: i32,
    pub property: WikidataUuidProp,
    pub value: uuid::Uuid,
}
