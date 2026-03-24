/// A user's tag vote on a visual novel.
///
/// `uid` is `None` for anonymous votes; `id` is a surrogate key because
/// `NULL ≠ NULL` prevents a composite natural key from enforcing uniqueness.
/// Negative `vote` means a downvote; 1–3 is an upvote.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct TagVn {
    pub id: i64,
    pub date: time::Date,
    pub tag_id: i32,
    pub vn_id: i32,
    pub uid: Option<i32>,
    pub vote: i16,
    /// Spoiler level 0–2; `None` when not applicable.
    pub spoiler: Option<i16>,
    /// When `true`, this vote is excluded from aggregates.
    pub ignore: bool,
    /// When `true`, implies `spoiler = 0`.
    pub lie: Option<bool>,
    pub notes: String,
}
