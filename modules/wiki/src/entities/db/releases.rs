use super::language::Language;

/// Release completeness type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "release_type", rename_all = "snake_case")]
pub enum ReleaseType {
    Complete,
    Partial,
    Trial,
}

/// Physical or digital storage medium for a release.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "medium", rename_all = "snake_case")]
pub enum Medium {
    Cd,
    Dvd,
    Gdr,
    Blr,
    Flp,
    Cas,
    Mrt,
    Mem,
    Umd,
    Nod,
    In,
    Dc,
    Otc,
}

/// Target platform for a release.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "platform", rename_all = "snake_case")]
pub enum Platform {
    Win,
    Dos,
    Lin,
    Mac,
    Ios,
    And,
    Dvd,
    Bdp,
    Fm7,
    Fm8,
    Fmt,
    Gba,
    Gbc,
    Msx,
    Nds,
    Nes,
    P88,
    P98,
    Pce,
    Pcf,
    Psp,
    Ps1,
    Ps2,
    Ps3,
    Ps4,
    Ps5,
    Psv,
    Drc,
    Smd,
    Scd,
    Sat,
    Sfc,
    Swi,
    Sw2,
    Wii,
    Wiu,
    N3d,
    Vnd,
    X1s,
    X68,
    Xb1,
    Xb3,
    Xbo,
    Xxs,
    Web,
    Tdo,
    Mob,
    Oth,
}

/// Category of a release package image.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "release_image_type", rename_all = "snake_case")]
pub enum ReleaseImageType {
    Pkgfront,
    Pkgback,
    Pkgcontent,
    Pkgside,
    Pkgmed,
    Dig,
}

/// A release of one or more visual novels.
///
/// `ani_story` and `ani_ero` are deprecated; animation detail is now stored in
/// the `ani_*_sp` / `ani_*_cg` columns. Animation flag columns use
/// `Option<i16>` to represent the `animation` domain (`NULL` = not applicable).
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Release {
    pub id: i32,
    /// JAN/UPC/EAN/ISBN barcode.
    pub gtin: i64,
    /// Language of the main display title.
    pub olang: Language,
    pub released: i32,
    pub voiced: i16,
    /// Horizontal resolution. When 0, `reso_y` encodes special values.
    pub reso_x: i16,
    pub reso_y: i16,
    /// Age rating 0–18.
    pub minage: Option<i16>,
    /// Deprecated: use `ani_story_sp` / `ani_story_cg` instead.
    pub ani_story: i16,
    /// Deprecated: use `ani_ero_sp` / `ani_ero_cg` instead.
    pub ani_ero: i16,
    pub ani_story_sp: Option<i16>,
    pub ani_story_cg: Option<i16>,
    pub ani_cutscene: Option<i16>,
    pub ani_ero_sp: Option<i16>,
    pub ani_ero_cg: Option<i16>,
    pub ani_bg: Option<bool>,
    pub ani_face: Option<bool>,
    pub has_ero: bool,
    pub patch: bool,
    pub freeware: bool,
    pub uncensored: Option<bool>,
    pub official: bool,
    pub catalog: String,
    pub notes: String,
    pub engine_id: Option<i32>,
}

/// A localised title row for a release.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct ReleaseTitle {
    pub release_id: i32,
    pub lang: Language,
    /// Machine-translated when `true`.
    pub mtl: bool,
    pub title: Option<String>,
    pub latin: Option<String>,
}

/// Association between a release and a VN, with a release type.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct ReleaseVn {
    pub release_id: i32,
    pub vn_id: i32,
    pub rtype: ReleaseType,
}

/// DRM applied to a release, with optional notes.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct ReleaseDrm {
    pub release_id: i32,
    pub drm_id: i32,
    pub notes: String,
}

/// An external link associated with a release.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct ReleaseExtLink {
    pub release_id: i32,
    pub link: i32,
}

/// A package image for a release (box art, disc scan, etc.).
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct ReleaseImage {
    pub release_id: i32,
    pub image_id: i32,
    pub vn_id: Option<i32>,
    pub itype: ReleaseImageType,
}

/// A language associated with a release package image.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct ReleaseImageLang {
    pub release_id: i32,
    pub image_id: i32,
    pub lang: Language,
}

/// A storage medium included in a release, with quantity.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct ReleaseMedium {
    pub release_id: i32,
    pub medium: Medium,
    pub qty: i16,
}

/// A target platform for a release.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct ReleasePlatform {
    pub release_id: i32,
    pub platform: Platform,
}

/// A producer's role (developer and/or publisher) for a release.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct ReleaseProducer {
    pub release_id: i32,
    pub producer_id: i32,
    pub developer: bool,
    pub publisher: bool,
}

/// Records that one release supersedes (replaces) another.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct ReleaseSupersedes {
    pub release_id: i32,
    pub superseded_id: i32,
}
