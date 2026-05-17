# RLU Lib (LRU Cache Library)

A highly efficient, thread-safe, and feature-rich Least Recently Used (LRU) cache library implemented in Rust.

## 🚀 Features

- **Dual-Mode Support**:
    - **Capacity-based**: Limits the number of entries in the cache.
    - **TTL-based**: Limits the lifetime of entries based on their last access.
    - **Hybrid**: Combines both capacity and TTL constraints.
- **Thread-Safe**: Built with `parking_lot::RwLock` for high-performance concurrent access.
- **Persistence**: Ability to save and restore cache state to/from disk (e.g., JSON format).
- **Extensible**: Uses traits for eviction policies and persistence adapters.

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rlu_lib = { git = "https://github.com/g-alexander/lru_lib.git" }
```

## 🛠 Usage

### Basic Example (Hybrid Mode)

```rust
use rlu_lib::cache::{Cache, LruCache};
use chrono::Duration;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Create a cache with capacity 100 and TTL 5 minutes
    let cache = Arc::new(LruCache::new(Some(100), Some(Duration::minutes(5))));

    // Put a value
    cache.put(1, "Hello, World!".to_string()).unwrap();

    // Get a value
    match cache.get(&1) {
        Ok(val) => println!("Found: {}", val),
        Err(_) => println!("Not found or expired"),
    }
}
```

### Persistence Example

```rust
use rlu_lib::cache::{Cache, LruCache};
use rlu_lib::persistence::json_adapter::JsonPersistenceAdapter;
use std::path::Path;

#[tokio::test]
async fn test_persistence() {
    let cache = LruCache::new(Some(10), None);
    let adapter = JsonPersistenceAdapter;
    let path = Path::new("cache.json");

    cache.put(1, "data".to_string()).unwrap();
    cache.save_to_disk(path, &adapter).await.unwrap();

    let new_cache = LruCache::new(Some(10), None);
    new_cache.load_from_disk(path, &adapter).await.unwrap();
    assert_eq!(new_cache.get(&1).unwrap(), "data");
}
```

## 🏗 Architecture

The library is designed around several key components:
- `Cache`: The core trait defining the interface.
- `LruCache`: The primary implementation of the cache.
- `PersistenceAdapter`: A trait for implementing different storage backends (e 
g., JSON, Bincode, etc.).
- `EvictionPolicy`: A trait for defining how elements are selected for eviction (e.g., LRU).
