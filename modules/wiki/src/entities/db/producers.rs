use super::language::Language;

/// Organisational form of a producer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "producer_type", rename_all = "snake_case")]
pub enum ProducerType {
    /// Company.
    Company,
    /// Individual.
    Individual,
    /// Non-profit / doujin group.
    NonProfit,
}

/// Relationship type between two producers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "producer_relation", rename_all = "snake_case")]
pub enum ProducerRelationType {
    OldName,
    NewName,
    Subsidiary,
    ParentCompany,
    Imprint,
    ImprintOfParent,
    SubsidiaryOfParent,
    OriginalBrand,
}

/// A game developer or publisher.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct Producer {
    pub id: i32,
    pub producer_type: ProducerType,
    pub lang: Language,
    pub name: String,
    pub latin: Option<String>,
    pub description: String,
}

/// An alternative name (alias) for a producer.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct ProducerAlias {
    pub producer_id: i32,
    pub alias: String,
}

/// A directional relationship between two producers.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct ProducerRelEntry {
    pub producer_id: i32,
    pub related_id: i32,
    pub relation: ProducerRelationType,
}

/// An external link associated with a producer.
#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct ProducerExtLink {
    pub producer_id: i32,
    pub link: i32,
}
