use super::language::Language;

/// Presented gender of a staff member.
///
/// The empty-string DB value (`""`) is mapped to `Unknown`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "staff_gender", rename_all = "snake_case")]
pub enum StaffGender {
    #[sqlx(rename = "")]
    Unknown,
    #[sqlx(rename = "m")]
    Male,
    #[sqlx(rename = "f")]
    Female,
}

/// A staff member (writer, artist, musician, etc.).
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Staff {
    pub id: i32,
    pub gender: StaffGender,
    pub lang: Language,
    /// Primary alias ID (`aid`) for this staff entry.
    pub main_alias_id: i32,
    pub description: String,
    /// Linked producer entry, if this staff member is also a company/group.
    pub producer_id: Option<i32>,
}

/// An alias (name variant) for a staff member.
///
/// `aid` is globally unique across all staff entries.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct StaffAlias {
    pub staff_id: i32,
    pub aid: i32,
    pub name: String,
    pub latin: Option<String>,
}

/// An external link associated with a staff member.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct StaffExtLink {
    pub staff_id: i32,
    pub link: i32,
}
