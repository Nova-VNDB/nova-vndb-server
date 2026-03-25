pub mod language;
pub mod entry_meta;
pub mod quotes;
pub mod anime;
pub mod wikidata;
pub mod game_tech_stack;
pub mod external_links;
pub mod image_resource;
pub mod tags;
pub mod traits_data;
pub mod producers;
pub mod staff;
pub mod character;
pub mod visual_novels;
pub mod releases;

/// A ternary flag for fields that can be affirmative, negative, or unknown.
///
/// Used instead of `Option<bool>` when the three states are semantically distinct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "bool_or_unknown", rename_all = "snake_case")]
pub enum BoolOrUnknown {
    True,
    False,
    Unknown,
}

/// Spoiler severity level, used across characters, traits, and tags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "spoil_level", rename_all = "snake_case")]
pub enum SpoilLevel {
    /// Not a spoiler.
    None,
    /// Reveals minor plot details.
    Minor,
    /// Reveals major plot details.
    Major,
}
