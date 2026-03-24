use super::language::Language;

/// Biological sex of a character.
///
/// The empty-string DB value (`""`) maps to `Unknown`.
/// `Both` means the character presents as both sexes; `NotApplicable` is for
/// non-biological entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "char_sex", rename_all = "snake_case")]
pub enum CharSex {
    #[sqlx(rename = "")]
    Unknown,
    #[sqlx(rename = "m")]
    Male,
    #[sqlx(rename = "f")]
    Female,
    #[sqlx(rename = "b")]
    Both,
    #[sqlx(rename = "n")]
    NotApplicable,
}

/// Presented gender identity of a character.
///
/// Distinct from `CharSex` (biological sex).
/// The empty-string DB value (`""`) maps to `Unknown`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "char_gender", rename_all = "snake_case")]
pub enum CharGender {
    #[sqlx(rename = "")]
    Unknown,
    #[sqlx(rename = "m")]
    Male,
    #[sqlx(rename = "f")]
    Female,
    #[sqlx(rename = "o")]
    Other,
    #[sqlx(rename = "a")]
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
}

/// Bra cup size of a character.
///
/// All DB values are uppercase, so each variant carries an explicit rename.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "cup_size", rename_all = "snake_case")]
pub enum CupSize {
    #[sqlx(rename = "")]
    Unknown,
    #[sqlx(rename = "AAA")]
    Aaa,
    #[sqlx(rename = "AA")]
    Aa,
    #[sqlx(rename = "A")]
    A,
    #[sqlx(rename = "B")]
    B,
    #[sqlx(rename = "C")]
    C,
    #[sqlx(rename = "D")]
    D,
    #[sqlx(rename = "E")]
    E,
    #[sqlx(rename = "F")]
    F,
    #[sqlx(rename = "G")]
    G,
    #[sqlx(rename = "H")]
    H,
    #[sqlx(rename = "I")]
    I,
    #[sqlx(rename = "J")]
    J,
    #[sqlx(rename = "K")]
    K,
    #[sqlx(rename = "L")]
    L,
    #[sqlx(rename = "M")]
    M,
    #[sqlx(rename = "N")]
    N,
    #[sqlx(rename = "O")]
    O,
    #[sqlx(rename = "P")]
    P,
    #[sqlx(rename = "Q")]
    Q,
    #[sqlx(rename = "R")]
    R,
    #[sqlx(rename = "S")]
    S,
    #[sqlx(rename = "T")]
    T,
    #[sqlx(rename = "U")]
    U,
    #[sqlx(rename = "V")]
    V,
    #[sqlx(rename = "W")]
    W,
    #[sqlx(rename = "X")]
    X,
    #[sqlx(rename = "Y")]
    Y,
    #[sqlx(rename = "Z")]
    Z,
}

/// A character entry.
///
/// `(main, main_spoil)` have been extracted to [`CharInstance`] (DKNF BCNF).
/// Physical measurements are stored in cm/kg with 0 meaning "unknown".
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Char {
    pub id: i32,
    pub image_id: Option<i32>,
    pub bloodt: BloodType,
    pub cup_size: CupSize,
    pub sex: CharSex,
    /// Actual sex when it is a plot spoiler.
    pub spoil_sex: Option<CharSex>,
    pub gender: Option<CharGender>,
    pub spoil_gender: Option<CharGender>,
    /// Bust measurement in cm.
    pub s_bust: i16,
    /// Waist measurement in cm.
    pub s_waist: i16,
    /// Hip measurement in cm.
    pub s_hip: i16,
    /// Birthday encoded as `0` (unknown) or `mmdd` (e.g. `1225` for Dec 25).
    pub birthday: i16,
    /// Height in cm.
    pub height: i16,
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
    pub main_spoil: i16,
}

/// An alias for a character, with an associated spoiler level.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct CharAlias {
    pub char_id: i32,
    pub spoil: i16,
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
    pub spoil: i16,
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
    pub spoil: i16,
}
