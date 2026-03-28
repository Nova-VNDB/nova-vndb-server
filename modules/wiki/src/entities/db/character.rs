use super::language::Language;
use crate::entities::db::SpoilLevel;
use time::Date;

/// Biological sex of a character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "char_sex", rename_all = "snake_case")]
pub enum CharSex {
    Unknown,
    Male,
    Female,
    Intersex,
    Neuter,
    Other,
}

/// Presented gender identity of a character.
///
/// Distinct from `CharSex` (biological sex).
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "char_gender", rename_all = "snake_case")]
pub enum CharGender {
    Unknown,
    Male,
    Female,
    NonBinary,
    Ambiguous,
}

/// Role a character plays within a visual novel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "char_role", rename_all = "snake_case")]
pub enum CharRole {
    Main,
    Primary,
    Side,
    Appears,
}

/// Blood type of a character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "blood_type", rename_all = "snake_case")]
pub enum BloodType {
    Unknown,
    A,
    B,
    Ab,
    O,
    Other,
}

/// Bra cup size of a character.
///
/// All DB values are uppercase, so each variant carries an explicit rename.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "cup_size", rename_all = "UPPERCASE")]
pub enum CupSize {
    Unknown,
    Aaa,
    Aa,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

/// A character entry.
///
/// `(main, main_spoil)` have been extracted to [CharInstance].
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Char {
    pub id: i32,
    pub image_id: Option<i32>,

    pub blood_type: Option<BloodType>,

    pub cup_size: Option<CupSize>,

    pub sex: Option<CharSex>,

    /// Actual sex when it is a plot spoiler.
    pub spoil_sex: Option<CharSex>,

    pub gender: Option<CharGender>,

    /// Actual gender when it is a plot spoiler.
    pub spoil_gender: Option<CharGender>,

    /// Bust measurement in cm.
    pub bust: Option<i16>,

    /// Waist measurement in cm.
    pub waist: Option<i16>,

    /// Hip measurement in cm.
    pub hip: Option<i16>,

    /// Birthday encoded as `0` (unknown) or `mmdd` (e.g. `1225` for Dec 25).
    pub birthday: Date,

    /// Height in cm.
    pub height: Option<i16>,

    /// Weight in kg.
    pub weight: Option<i16>,

    /// Age in years.
    pub age: Option<i16>,

    pub description: String,
}

/// Records that a character is an alternate instance of another character.
///
/// Extracted from `chars.(main, main_spoil)` to satisfy DKNF (BCNF):
/// `main_spoil` is only meaningful when `main` is present.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct CharInstance {
    pub char_id: i32,
    pub main_id: i32,
    pub main_spoil: SpoilLevel,
}

/// An alias for a character, with an associated spoiler level.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct CharAlias {
    pub char_id: i32,
    pub spoil: SpoilLevel,
    pub name: String,
    pub latin: Option<String>,
}

/// A localised name for a character.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct CharName {
    pub char_id: i32,
    pub lang: Language,
    pub name: String,
    pub latin: Option<String>,
}

/// Association between a character and a trait.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct CharTrait {
    pub char_id: i32,
    pub trait_id: i32,
    pub spoil: SpoilLevel,
    /// When `true`, the character only pretends to have this trait.
    pub lie: bool,
}

/// The role a character plays in a specific VN (and optionally a release).
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct CharVn {
    pub char_id: i32,
    pub vn_id: i32,
    pub release_id: Option<i32>,
    pub role: CharRole,
    pub spoil: SpoilLevel,
}
