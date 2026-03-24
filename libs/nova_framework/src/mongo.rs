use tracing::info;

#[derive(Debug, Clone)]
pub struct MongoProcessor {
    executor: mongodb::Client,
}

impl MongoProcessor {
    pub fn new(executor: mongodb::Client) -> Self {
        Self { executor }
    }
    pub fn db(&self) -> &mongodb::Client {
        info!(monotonic_counter.sql = 1);
        &self.executor
    }
}
