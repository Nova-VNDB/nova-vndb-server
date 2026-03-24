use tracing::info;

#[derive(Debug, Clone)]
pub struct DatabaseProcessor {
    executor: sqlx::PgPool,
}

impl DatabaseProcessor {
    pub fn new(executor: sqlx::PgPool) -> Self {
        Self { executor }
    }
}

impl DatabaseProcessor {
    pub fn db(&self) -> &sqlx::PgPool {
        info!(monotonic_counter.sql = 1);
        &self.executor
    }
}
