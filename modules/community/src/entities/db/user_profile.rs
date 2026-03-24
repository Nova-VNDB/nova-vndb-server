/// A VNDB community user account.
///
/// Stores permission flags and the public-facing username.
/// Authentication credentials are managed by the `auth` module.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct CommunityUser {
    pub id: i32,
    /// When `true`, all votes by this user are globally ignored.
    pub ign_votes: bool,
    /// When `false`, this user's image votes do not count.
    pub perm_imgvote: bool,
    /// When `false`, this user's tag votes do not count.
    pub perm_tag: bool,
    /// When `false`, this user's length votes do not count.
    pub perm_lengthvote: bool,
    pub username: Option<String>,
}

/// A label (built-in or user-defined) that can be applied to VN list entries.
///
/// Label IDs: `0 < builtin < 10 ≤ custom`.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct UlistLabel {
    pub uid: i32,
    pub label_id: i16,
    pub label: String,
}

/// A visual novel entry in a user's reading list.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct UlistVn {
    pub uid: i32,
    pub vn_id: i32,
    pub added: time::Date,
    pub lastmod: time::Date,
    pub vote_date: Option<time::Date>,
    pub started: Option<time::Date>,
    pub finished: Option<time::Date>,
    /// Vote score 10–100.
    pub vote: Option<i16>,
    pub notes: String,
}

/// Associates a label with a user's VN list entry.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct UlistVnLabel {
    pub uid: i32,
    pub vn_id: i32,
    pub label_id: i16,
}

/// A physical or digital release in a user's collection.
///
/// Status: 0 = Unknown, 1 = Pending, 2 = Obtained, 3 = On loan, 4 = Deleted.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Rlist {
    pub uid: i32,
    pub release_id: i32,
    pub added: time::Date,
    pub status: i16,
}
