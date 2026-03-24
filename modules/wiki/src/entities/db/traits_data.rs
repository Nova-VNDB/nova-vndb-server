/// A character trait that can be applied to characters.
///
/// `gid` caches the root group trait for display purposes.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Trait {
    pub id: i32,
    /// Trait group (cached: main parent's root trait).
    pub gid: Option<i32>,
    /// Group order; only meaningful when `gid` is `None`.
    pub gorder: i16,
    pub defaultspoil: i16,
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
