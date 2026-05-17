use crate::error::Result;
use crate::models::CacheEntry;
use crate::persistence::PersistenceAdapter;
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufWriter};

/// A JSON persistence adapter for the cache.
pub struct JsonPersistenceAdapter;

#[async_trait]
impl<K, V> PersistenceAdapter<K, V> for JsonPersistenceAdapter
where
    K: serde::Serialize + serde::de::DeserializeOwned + Send + Sync + std::hash::Hash + Eq + Clone,
    V: serde::Serialize + serde::de::DeserializeOwned + Send + Sync + Clone,
{
    /// Saves the cache state to a file in JSON format.
    async fn save(&self, path: &Path, data: &HashMap<K, CacheEntry<V>>) -> Result<()> {
        let file = File::create(path).map_err(|e| crate::error::CacheError::Io(e))?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, data)
            .map_err(|e| crate::error::CacheError::Serialization(e.to_string()))?;
        Ok(())
    }

    /// Loads the cache state from a JSON file.
    async fn load(&self, path: &Path) -> Result<HashMap<K, CacheEntry<V>>> {
        let file = File::open(path).map_err(|e| crate::error::CacheError::Io(e))?;
        let reader = BufReader::new(file);
        let data = serde_json::from_reader(reader)
            .map_err(|e| crate::error::CacheError::Serialization(e.to_string()))?;
        Ok(data)
    }
}