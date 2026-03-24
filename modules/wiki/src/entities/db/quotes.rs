/// A notable quote from a visual novel, optionally attributed to a character.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Quote {
    pub id: i32,
    pub vn_id: i32,
    pub char_id: Option<i32>,
    pub score: i16,
    pub quote: String,
}
