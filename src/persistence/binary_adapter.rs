use crate::error::Result;
use crate::models::CacheEntry;
use crate::persistence::PersistenceAdapter;
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use bincode;

/// A persistence adapter that uses the bincode format for efficient binary storage.
pub struct BinaryPersistenceAdapter;

#[async_trait]
impl<K, V> PersistenceAdapter<K, V> for BinaryPersistenceAdapter
where
    K: serde::Serialize + serde::de::DeserializeOwned + Send + Sync + std::hash::Hash + Eq + Clone,
    V: serde::Serialize + serde::de::DeserializeOwned + Send + Sync + Clone,
{
    /// Saves the cache state to a file in binary format using bincode.
    async fn save(&self, path: &Path, data: &HashMap<K, CacheEntry<V>>) -> Result<()> {
        let file = File::create(path).map_err(crate::error::CacheError::from)?;
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, data)
            .map_err(|e| crate::error::CacheError::Internal(e.to_string()))?;
        Ok(())
    }

    /// Loads the cache state from a binary file using bincode.
    async fn load(&self, path: &Path) -> Result<HashMap<K, CacheEntry<V>>> {
        let file = File::open(path).map_err(|_| crate::error::CacheError::NotFound)?;
        let reader = BufReader::new(file);
        let data: HashMap<K, CacheEntry<V>> = bincode::deserialize_from(reader)
            .map_err(|e| crate::error::CacheError::Internal(e.to_string()))?;
        Ok(data)
    }
}