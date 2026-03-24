#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "image_type", rename_all = "snake_case")]
pub enum ImageType {
    CoverImage,
    Character,
    Screenshot,
}

/// An image asset stored by VNDB.
///
/// Cached aggregate columns (vote counts, averages) are omitted; they are
/// derivable from the `image_votes` table via views.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Image {
    pub id: i32,
    pub image_type: ImageType,
    /// Width in pixels.
    pub width: i16,
    /// Height in pixels.
    pub height: i16,
}
