use uuid::Uuid;

#[derive(Debug, Copy, Clone, sqlx::FromRow)]
pub struct UserAttributes {
    pub user_id: Uuid,
    pub community_attribute: CommunityAttribute,
    pub wiki_attribute: WikiAttribute,
    pub sponsorship: Sponsorship,
    pub site_administrator: SiteAdministrator,
    pub verified_user: VerifiedUser,
    pub updated_at: time::PrimitiveDateTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
pub enum CommunityAttribute {
    Suspended,
    Normal,
    CommunityModerator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
pub enum WikiAttribute {
    TrustedEditor,
    Editor,
    Contributor,
    EditingSuspended,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
pub enum Sponsorship {
    PremiumSponsor,
    Sponsor,
    BackupSponsor,
    NotSponsor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
pub enum SiteAdministrator {
    Owner,
    Admin,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
pub enum VerifiedUser {
    NotVerified,
    OfficialAccount,
    VerifiedCommenter,
    VerifiedEmployee,
    VerifierIndependentCreator,
    SiteAdministrator,
}
