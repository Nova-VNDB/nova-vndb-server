use super::language::Language;

/// Relationship type between two visual novels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "vn_relation", rename_all = "snake_case")]
pub enum VnRelationType {
    Sequel,
    Prequel,
    SameSetting,
    AlternativeVersion,
    SharesCharacters,
    SideStory,
    ParentStory,
    SameSeries,
    Fandisc,
    OriginalGame,
}

/// Staff credit type on a visual novel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "credit_type", rename_all = "snake_case")]
pub enum CreditType {
    Scenario,
    CharacterDesign,
    Art,
    Music,
    Songs,
    Director,
    Translator,
    Editor,
    QualityAssurance,
    Staff,
}

/// Legacy play-length rating for a VN.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "vn_length", rename_all = "snake_case")]
pub enum VnLength {
    Unknown,
    VeryShort,
    Short,
    Medium,
    Long,
    VeryLong,
}

/// Development / release status of a visual novel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "development_status", rename_all = "snake_case")]
pub enum DevelopmentStatus {
    Finished,
    Ongoing,
    Cancelled,
}

/// Self-reported reading speed when submitting a play-length vote.
///
/// `None` means the voter did not report their speed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "reading_speed", rename_all = "snake_case")]
pub enum ReadingSpeed {
    Slow,
    Normal,
    Fast,
}

/// A visual novel entry.
///
/// Cached/computed columns (`c_image`, `c_votecount`, etc.) are omitted;
/// they are derivable from source tables via views.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Vn {
    pub id: i32,
    /// Original language of the VN.
    pub original_language: Language,
    /// Legacy length rating.
    pub length: VnLength,
    pub development_status: DevelopmentStatus,
    pub description: String,
}

/// A localised title for a visual novel.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct VnTitle {
    pub vn_id: i32,
    pub lang: Language,
    pub official: bool,
    pub title: String,
    pub latin: Option<String>,
}

/// An alternative title / alias for a visual novel.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct VnAlias {
    pub vn_id: i32,
    pub alias: String,
}

/// A directional relation between two VNs.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct VnRelationEntry {
    pub vn_id: i32,
    pub related_id: i32,
    pub relation: VnRelationType,
    pub official: bool,
}

/// Cross-reference between a VN and an anime adaptation.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct VnAnime {
    pub vn_id: i32,
    pub anime_id: i32,
}

/// A localisation edition of a visual novel.
///
/// `edition_id` is local to the VN and is not stable across revisions.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct VnEdition {
    pub vn_id: i32,
    pub lang: Option<Language>,
    pub edition_id: i16,
    pub official: bool,
    pub name: String,
}

/// An external link associated with a VN.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct VnExtLink {
    pub vn_id: i32,
    pub link: i32,
}

/// A screenshot associated with a VN, optionally tied to a release.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct VnScreenshot {
    pub vn_id: i32,
    pub image_id: i32,
    pub release_id: Option<i32>,
}

/// A user's vote for which image should be the cover of a VN.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct VnImageVote {
    pub vn_id: i32,
    pub user_id: i32,
    pub image_id: i32,
}

/// A staff credit on a VN, optionally scoped to an edition.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct VnStaff {
    pub vn_id: i32,
    pub alias_id: i32,
    pub role: CreditType,
    pub edition_id: Option<i16>,
    pub note: String,
}

/// A voice-acting credit linking a staff alias to a character in a VN.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct VnSeiyuu {
    pub vn_id: i32,
    pub char_id: i32,
    pub alias_id: i32,
    pub note: String,
}

/// A user's estimated play-length vote for a VN.
///
/// `user_id` is `None` for anonymous votes.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct VnLengthVote {
    pub vote_id: i64,
    pub vn_id: i32,
    pub user_id: Option<i32>,
    pub date: time::Date,
    /// Estimated play length in minutes.
    pub length: i16,
    /// `None` means the voter did not report their reading speed.
    pub speed: Option<ReadingSpeed>,
    pub notes: String,
}

/// A release associated with a play-length vote.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct VnLengthVoteRelease {
    pub vote_id: i64,
    pub release_id: i32,
}

/// A language associated with a play-length vote.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct VnLengthVoteLang {
    pub vote_id: i64,
    pub lang: Language,
}
