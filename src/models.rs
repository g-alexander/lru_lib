use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Represents an entry in the cache, containing the value and metadata for expiration and access tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry<V> {
    /// The actual value stored in the cache.
    pub value: V,
    /// The last time this entry was accessed.
    pub last_accessed: DateTime<Utc>,
    /// The time when this entry should expire.
    pub expires_at: Option<DateTime<Utc>>,
}

impl<V> CacheEntry<V>
where
    V: Clone,
{
    /// Creates a new `CacheEntry`.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store.
    /// * `ttl` - Optional Time-To-Live duration. If provided, `expires_at` is set.
    pub fn new(value: V, ttl: Option<chrono::Duration>) -> Self {
        let now = Utc::now();
        let expires_at = ttl.map(|d| now + d);
        
        Self {
            value,
            last_accessed: now,
            expires_at,
        }
    }

    /// Checks if the entry has expired based on its `expires_at` timestamp.
    pub fn is_expired(&self) -> bool {
        if let Some(expiry) = self.expires_at {
            return Utc::now() > expiry;
        }
        false
    }

    /// Updates the `last_accessed` timestamp to the current time.
    pub fn touch(&mut self) {
        self.last_accessed = Utc::now();
    }
}