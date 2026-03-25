use super::language::Language;
use crate::entities::db::BoolOrUnknown;

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
    GdRom,
    BluRayDisc,
    Floppy,
    CassetteTape,
    Cartridge,
    MemoryCard,
    Umd,
    NintendoOpticalDisc,
    InternetDownload,
    DownloadCard,
    Other,
}

/// Target platform for a release.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "platform", rename_all = "snake_case")]
pub enum Platform {
    Windows,
    Dos,
    Linux,
    MacOs,
    AppleIProduct,
    Android,
    DvdPlayer,
    BluRayPlayer,
    Fm7,
    Fm8,
    FmTowns,
    GameBoyAdvance,
    GameBoyColor,
    Msx,
    NintendoDs,
    Famicom,
    Pc88,
    Pc98,
    PcEngine,
    PcFx,
    PlayStationPortable,
    PlayStation1,
    PlayStation2,
    PlayStation3,
    PlayStation4,
    PlayStation5,
    PlayStationVita,
    Dreamcast,
    SegaMegaDrive,
    SegaMegaCd,
    SegaSaturn,
    SuperFamicom,
    NintendoSwitch,
    NintendoSwitch2,
    NintendoWii,
    NintendoWiiU,
    Nintendo3Ds,
    Vnds,
    SharpX1,
    SharpX68000,
    Xbox,
    Xbox360,
    XboxOne,
    XboxXs,
    Website,
    ThreeDo,
    OtherMobile,
    Other,
}

/// Category of a release package image.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "release_image_type", rename_all = "snake_case")]
pub enum ReleaseImageType {
    #[sqlx(rename = "pkg_front")]
    PackageFront,
    #[sqlx(rename = "pkg_back")]
    PackageBack,
    #[sqlx(rename = "pkg_content")]
    PackageContent,
    #[sqlx(rename = "pkg_side")]
    PackageSide,
    #[sqlx(rename = "pkg_medium")]
    PackageMedium,
    #[sqlx(rename = "dig")]
    Digital,
}

/// Voicing coverage of a release.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "voiced_level", rename_all = "snake_case")]
pub enum VoicedLevel {
    NotApplicable,
    NotVoiced,
    EroScenesOnly,
    PartiallyVoiced,
    FullyVoiced,
}

/// Animation quality level for a particular category of scenes.
///
/// `None` (stored as NULL) means animation data is not applicable for this release.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "animation_level", rename_all = "snake_case")]
pub enum AnimationLevel {
    /// Basic looping animations (e.g. blinking, lip sync).
    Simple,
    /// Some scenes have animation sequences.
    Partial,
    /// All or most scenes are fully animated.
    Full,
}

/// A release of one or more visual novels.
///
/// Cached/computed columns are omitted; `animation_background` and
/// `animation_face` use `BoolOrUnknown` because they are binary flags
/// (animated or not) rather than graded levels.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Release {
    pub id: i32,
    /// JAN/UPC/EAN/ISBN barcode.
    pub gtin: i64,
    /// Language of the main display title.
    pub original_language: Language,
    pub released: i32,
    pub voiced: VoicedLevel,

    /// Horizontal resolution. When 0, `resolution_y` encodes special values.
    pub resolution_x: Option<i16>,
    pub resolution_y: Option<i16>,

    /// Age rating 0–18.
    pub age_rating: Option<i16>,

    #[sqlx(rename = "ani_story_sp")]
    pub animation_story_sprite: Option<AnimationLevel>,
    #[sqlx(rename = "ani_story_cg")]
    pub animation_story_cg: Option<AnimationLevel>,
    #[sqlx(rename = "ani_cutscene")]
    pub animation_cutscene: Option<AnimationLevel>,
    #[sqlx(rename = "ani_ero_sp")]
    pub animation_erotic_sprite: Option<AnimationLevel>,
    #[sqlx(rename = "ani_ero_cg")]
    pub animation_erotic_cg: Option<AnimationLevel>,
    #[sqlx(rename = "ani_bg")]
    pub animation_background: BoolOrUnknown,
    #[sqlx(rename = "ani_face")]
    pub animation_face: BoolOrUnknown,
    #[sqlx(rename = "has_ero")]
    pub has_erotic_content: bool,
    pub patch: bool,
    pub freeware: bool,
    pub uncensored: BoolOrUnknown,
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
    #[sqlx(rename = "mtl")]
    pub machine_translated: bool,
    pub title: Option<String>,
    pub latin: Option<String>,
}

/// Association between a release and a VN, with a release type.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct ReleaseVn {
    pub release_id: i32,
    pub vn_id: i32,
    #[sqlx(rename = "rtype")]
    pub release_type: ReleaseType,
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
    #[sqlx(rename = "itype")]
    pub image_type: ReleaseImageType,
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
    #[sqlx(rename = "qty")]
    pub quantity: i16,
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
