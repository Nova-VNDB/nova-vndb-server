/// A DRM type that can be associated with a release.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Drm {
    pub id: i32,
    pub disc: bool,
    #[sqlx(rename = "cdkey")]
    pub cd_key: bool,
    pub activate: bool,
    #[sqlx(rename = "alimit")]
    pub activation_limit: bool,
    pub account: bool,
    pub online: bool,
    pub cloud: bool,
    pub physical: bool,
    pub name: String,
    pub description: String,
}

/// A VN engine that can be associated with a release.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Engine {
    pub id: i32,
    pub name: String,
    pub description: String,
}
