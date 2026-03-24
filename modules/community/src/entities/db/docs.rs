/// A documentation page.
///
/// Content is stored in MultiMarkdown format.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Doc {
    pub id: i32,
    pub title: String,
    pub content: String,
}
