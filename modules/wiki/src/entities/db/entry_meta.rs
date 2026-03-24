/// Discriminator for the `entry_meta` table, replacing the entity-type prefix
/// that was previously encoded in VNDB string IDs (e.g. `v` for VNs, `c` for chars).
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "entity_type", rename_all = "snake_case")]
pub enum EntityType {
    Vn,
    Char,
    Producer,
    Release,
    Staff,
    Doc,
    Quote,
    Tag,
    Trait,
}

/// Shared editorial metadata for all wiki entry types.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct EntryMeta {
    pub entity_type: EntityType,
    pub id: i32,
    pub created: time::Date,
    pub lastmod: time::Date,
    pub revision: i16,
    pub num_edits: i16,
    pub num_users: i16,
}
