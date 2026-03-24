/// A content-rating vote cast on an image.
///
/// `uid` is `None` for anonymous votes.
/// Scores: sexual 0 = safe / 1 = suggestive / 2 = explicit;
///         violence 0 = tame / 1 = violent / 2 = brutal.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct ImageVote {
    pub image_id: i32,
    pub uid: Option<i32>,
    pub date: time::Date,
    pub sexual: i16,
    pub violence: i16,
    /// Set when a moderator has overruled this vote.
    pub ignore: bool,
}
