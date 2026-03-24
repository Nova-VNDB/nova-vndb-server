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

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
pub enum BoolOrUnknown {
    True,
    False,
    Unknown,
}