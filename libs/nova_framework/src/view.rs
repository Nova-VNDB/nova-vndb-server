use crate::mongo::MongoProcessor;

/// A trait that marks a struct participates in view derivation.
pub trait DeriveView<View: CachedView> {
    fn find_views(
        &self,
        mongo_processor: MongoProcessor,
    ) -> impl Future<Output = Result<Vec<View>, crate::error::Error>>;
}

/// A view stored in MongoDB and can be derived from a full dependency.
pub trait CachedView {
    type Dependency;
    type Identifier;
    fn derive_from_full_dependency(
        dependency: Self::Dependency,
    ) -> Result<Self, crate::error::Error>
    where
        Self: Sized;
}

/// A view that can be partially updated from a partial dependency update.
pub trait PartialUpdateView<Part: DeriveView<Self>>: CachedView + Sized {
    fn derive_from_partial_dependency(
        new_dependency: Part,
        old_dependency: Part,
        old: &Self,
    ) -> Result<Self, crate::error::Error>;
}

#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
/// A signal to update a view.
///
/// This cannot be used as a message directly, use newtype pattern and `kanau::RkyvMessageSer`/`kanau::RkyvMessageDe` to make it usable in message passing.
pub struct ViewUpdateSignal<View, Dep>
where
    View: CachedView,
    View::Identifier: Send + Sync,
    Dep: Send + Sync,
{
    id: View::Identifier,
    old: Dep,
    new: Dep,
}
