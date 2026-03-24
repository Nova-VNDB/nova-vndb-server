use thiserror::Error;

#[derive(Debug, Error)]
/// internal errors
pub enum Error {
    #[error("{0}")]
    /// Serialize Error
    SerializeError(#[from] kanau::message::SerializeError),

    /// AMQP Error
    #[error("{0}")]
    AmqpError(#[from] amqprs::error::Error),

    /// Redis Error
    #[error("{0}")]
    RedisError(#[from] redis::RedisError),

    #[error("{0}")]
    /// Deserialize Error
    DeserializeError(#[from] kanau::message::DeserializeError),

    #[error("{0}")]
    /// Database Error
    DatabaseError(#[from] sqlx::Error),
    
    #[error("{0}")]
    /// MongoDB Error
    MongoError(#[from] mongodb::error::Error),

    #[error("{0}")]
    /// Error occurred in business logic. This kind of business error can not be solved by retrying.
    BusinessPanic(anyhow::Error),

    #[error("{0}")]
    /// IO Error occurred in business logic. This kind of error can be solved by just retrying.
    Io(anyhow::Error),

    #[error("Permission is not enough")]
    /// Trying to do some operation that requires higher permission
    PermissionsDenied,

    #[error("Invalid input")]
    InvalidInput,

    #[error("Trying to access a resource that does not exist")]
    NotFound,
}
