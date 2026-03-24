use super::language::Language;

/// Relationship type between two visual novels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "vn_relation", rename_all = "snake_case")]
pub enum VnRelationType {
    Seq,
    Preq,
    Set,
    Alt,
    Char,
    Side,
    Par,
    Ser,
    Fan,
    Orig,
}

/// Staff credit type on a visual novel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "credit_type", rename_all = "snake_case")]
pub enum CreditType {
    Scenario,
    Chardesign,
    Art,
    Music,
    Songs,
    Director,
    Translator,
    Editor,
    Qa,
    Staff,
}

/// A visual novel entry.
///
/// Cached/computed columns (`c_image`, `c_votecount`, etc.) are omitted;
/// they are derivable from source tables via views.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Vn {
    pub id: i32,
    /// Original language of the VN.
    pub olang: Language,
    /// Legacy length rating: 0 = unknown … 5 = very long.
    pub length: i16,
    /// Development status: 0 = finished, 1 = ongoing, 2 = cancelled.
    pub devstatus: i16,
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
/// `eid` is local to the VN and is not stable across revisions.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct VnEdition {
    pub vn_id: i32,
    pub lang: Option<Language>,
    pub eid: i16,
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
    pub uid: i32,
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
/// `uid` is `None` for anonymous votes.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct VnLengthVote {
    pub vote_id: i64,
    pub vn_id: i32,
    pub uid: Option<i32>,
    pub date: time::Date,
    /// Estimated play length in minutes.
    pub length: i16,
    /// Reader speed: `None` = uncounted; 0 = slow; 1 = normal; 2 = fast.
    pub speed: Option<i16>,
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
