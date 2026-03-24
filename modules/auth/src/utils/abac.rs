use crate::entities::db::user_attribute::UserAttributes;
use kanau::processor::{IdentityFunctor, Processor};
use nova_framework::sqlx::DatabaseProcessor;

pub trait Abac {
    const OPERATION_NAME: &'static str;
    const PERMISSION_NAME: &'static str;
    fn is_allowed(&self, attributes: UserAttributes) -> bool;
    fn with_user(self, user: UserAttributes) -> AuthenticatedOperation<Self>
    where
        Self: Sized,
    {
        AuthenticatedOperation {
            operation: self,
            user,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthenticatedOperation<T: Abac> {
    pub operation: T,
    pub user: UserAttributes,
}

#[derive(Debug, Clone)]
pub struct AbacLayer {
    database_processor: DatabaseProcessor,
}

impl AbacLayer {
    pub fn new(database_processor: DatabaseProcessor) -> Self {
        Self { database_processor }
    }
    pub fn into_adapter(
        self,
    ) -> kanau::layer::Adapter<Self, IdentityFunctor<nova_framework::Error>> {
        kanau::layer::Adapter::new(self, IdentityFunctor::new())
    }
}

impl<Oper> Processor<AuthenticatedOperation<Oper>> for AbacLayer
where
    Oper: Abac + Send,
{
    type Output = Oper;
    type Error = nova_framework::Error;
    async fn process(
        &self,
        input: AuthenticatedOperation<Oper>,
    ) -> Result<Oper, nova_framework::Error> {
        todo!()
    }
}
