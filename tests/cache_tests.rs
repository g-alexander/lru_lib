use lru_lib::cache::{Cache, LruCache};

#[tokio::test]
async fn test_cache_interface_methods() {
    let cache = LruCache::<i32, String>::new(Some(5), None);
    
    assert_eq!(cache.len(), 0);
    assert!(!cache.contains(&1).unwrap());
    cache.put(1, "val1".to_string()).unwrap();
    assert_eq!(cache.len(), 1);
    assert!(cache.contains(&1).unwrap());
    assert!(!cache.contains(&2).unwrap());
    cache.put(2, "val2".to_string()).unwrap();
    assert_eq!(cache.len(), 2);
    cache.remove(&1).unwrap();
    assert_eq!(cache.len(), 1);
    assert!(!cache.contains(&1).unwrap());
}