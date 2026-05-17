pub mod json_adapter;
pub mod binary_adapter;
pub use json_adapter::JsonPersistenceAdapter;
pub use binary_adapter::BinaryPersistenceAdapter;

use crate::error::Result;
use async_trait::async_trait;
use std::path::Path;

/// Trait for persistence adapters.
#[async_trait]
pub trait PersistenceAdapter<K, V>
where
    K: serde::Serialize + serde::de::DeserializeOwned + Send + Sync + std::hash::Hash + Eq + Clone,
    V: serde::Serialize + serde::de::DeserializeOwned + Send + Sync + Clone,
{
    /// Saves the cache state to a file.
    async fn save(&self, path: &Path, data: &std::collections::HashMap<K, crate::models::CacheEntry<V>>) -> Result<()>;

    /// Loads the cache state from a file.
    async fn load(&self, path: &Path) -> Result<std::collections::HashMap<K, crate::models::CacheEntry<V>>>;
}
