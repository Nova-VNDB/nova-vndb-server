/// Broad category that a tag belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "tag_category", rename_all = "snake_case")]
pub enum TagCategory {
    Cont,
    Ero,
    Tech,
}

/// A community-defined tag that can be applied to visual novels.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Tag {
    pub id: i32,
    pub cat: TagCategory,
    pub defaultspoil: i16,
    pub searchable: bool,
    pub applicable: bool,
    pub name: String,
    pub description: String,
}

/// An alternative name (alias) for a tag.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct TagAlias {
    pub tag_id: i32,
    pub alias: String,
}

/// A parent–child relationship in the tag hierarchy.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct TagParent {
    pub tag_id: i32,
    pub parent_id: i32,
    /// When `true`, this is the primary (canonical) parent.
    pub main: bool,
}
