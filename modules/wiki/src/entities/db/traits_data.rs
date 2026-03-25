/// A character trait that can be applied to characters.
///
/// `group_id` caches the root group trait for display purposes.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Trait {
    pub id: i32,
    /// Trait group (cached: main parent's root trait).
    #[sqlx(rename = "gid")]
    pub group_id: Option<i32>,
    /// Group order; only meaningful when `group_id` is `None`.
    #[sqlx(rename = "gorder")]
    pub group_order: i16,
    #[sqlx(rename = "defaultspoil")]
    pub default_spoil_level: i16,
    pub sexual: bool,
    pub searchable: bool,
    pub applicable: bool,
    pub name: String,
    pub description: String,
}

/// An alternative name (alias) for a trait.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct TraitAlias {
    pub trait_id: i32,
    pub alias: String,
}

/// A parent–child relationship in the trait hierarchy.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct TraitParent {
    pub trait_id: i32,
    pub parent_id: i32,
    /// When `true`, this is the primary (canonical) parent.
    pub main: bool,
}
