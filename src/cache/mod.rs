pub mod policy;
pub mod storage;
 
use crate::error::{CacheError, Result};
use crate::models::CacheEntry;
use crate::cache::policy::{EvictionPolicy, LruPolicy};
use crate::cache::storage::MemoryStorage;
use crate::persistence::PersistenceAdapter;
use std::hash::Hash;
use parking_lot::RwLock;
use std::path::Path;

/// The core trait for cache implementations.
pub trait Cache<K, V> 
where 
    K: Hash + Eq + Clone,
{
    /// Retrieves a value from the cache.
    fn get(&self, key: &K) -> Result<V>;
    /// Inserts a value into the cache.
    fn put(&self, key: K, value: V) -> Result<()>;
    /// Removes a specific key from the cache.
    fn remove(&self, key: &K) -> Result<()>;
    /// Clears all entries from the cache.
    fn clear(&self) -> Result<()>;
}

/// A thread-safe LRU cache implementation supporting both capacity-based and TTL-based eviction.
pub struct LruCache<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    capacity: Option<usize>,
    ttl: Option<chrono::Duration>,
    storage: RwLock<MemoryStorage<K, V>>,
    policy: RwLock<LruPolicy<K>>,
}

impl<K, V> LruCache<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    /// Creates a new `LruCache` instance.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Maximum number of entries allowed in the cache (optional).
    /// * `ttl` - Time-to-live duration for each entry (optional).
    pub fn new(capacity: Option<usize>, ttl: Option<chrono::Duration>) -> Self {
        Self {
            capacity,
            ttl,
            storage: RwLock::new(MemoryStorage::new()),
            policy: RwLock::new(LruPolicy::<K>::new()),
        }
    }

    /// Saves the cache state to a file using the provided adapter.
    pub async fn save_to_disk<A>(&self, path: &Path, adapter: &A) -> Result<()>
    where
        A: PersistenceAdapter<K, V>,
        K: serde::Serialize + serde::de::DeserializeOwned + Send + Sync,
        V: serde::Serialize + serde::de::DeserializeOwned + Send + Sync,
    {
        let storage = self.storage.read();
        adapter.save(path, &storage.data_as_map()).await
    }

    /// Restores the cache state from a file using the provided adapter.
    pub async fn load_from_disk<A>(&self, path: &Path, adapter: &A) -> Result<()>
    where
        A: PersistenceAdapter<K, V>,
        K: serde::Serialize + serde::de::DeserializeOwned + Send + Sync,
        V: serde::Serialize + serde::de::DeserializeOwned + Send + Sync,
    {
        let data = adapter.load(path).await?;
        let mut storage = self.storage.write();
        let mut policy = self.policy.write();

        for (key, entry) in data {
            storage.insert(key.clone(), entry)?;
            policy.on_insert(key);
        }
        Ok(())
    }
}

impl<K, V> Cache<K, V> for LruCache<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    fn get(&self, key: &K) -> Result<V> {
        let mut storage = self.storage.write();
        let mut policy = self.policy.write();
        if let Some(entry) = storage.get_mut(key) {
            if entry.is_expired() {
                storage.remove(key);
                policy.on_remove(key);
                return Err(CacheError::NotFound);
            }
            entry.touch();
            policy.on_access(key);
            Ok(entry.value.clone())
        } else {
            Err(CacheError::NotFound)
        }
    }
    fn put(&self, key: K, value: V) -> Result<()> {
        let mut storage = self.storage.write();
        let mut policy = self.policy.write();
        if let Some(entry) = storage.get_mut(&key) {
            let new_entry = CacheEntry::new(value, self.ttl);
            *entry = new_entry;
            policy.on_access(&key);
            return Ok(());
        }
        if let Some(cap) = self.capacity {
            if storage.len() >= cap {
                if let Some(evict_key) = policy.next_eviction() {
                    storage.remove(&evict_key);
                }
            }
        }
        let entry = CacheEntry::new(value, self.ttl);
        storage.insert(key.clone(), entry)?;
        policy.on_insert(key);
        Ok(())
    }
    fn remove(&self, key: &K) -> Result<()> {
        let mut storage = self.storage.write();
        let mut policy = self.policy.write();
        storage.remove(key);
        policy.on_remove(key);
        Ok(())
    }
    fn clear(&self) -> Result<()> {
        let mut storage = self.storage.write();
        let mut policy = self.policy.write();
        storage.clear();
        *policy = LruPolicy::new();
        Ok(())
    }
}