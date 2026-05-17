use lru_lib::cache::{Cache, LruCache};
use std::time::Duration as StdDuration;
use chrono::Duration as ChronoDuration;
use std::thread;

#[test]
fn test_hybrid_mode_capacity_and_ttl() {
    // Кэш с емкостью 2 и TTL 300мс
    let capacity = Some(2);
    let ttl = Some(ChronoDuration::milliseconds(300));
    let cache = LruCache::new(capacity, ttl);
    
    cache.put(1, "a".to_string()).unwrap();
    cache.put(2, "b".to_string()).unwrap();
    
    // Проверяем, что оба на месте
    assert_eq!(cache.get(&1).unwrap(), "a");
    assert_eq!(cache.get(&2).unwrap(), "b");
    
    // 1. Проверка емкости: добавляем 3-й элемент, 1-й должен удалиться (LRU)
    cache.put(3, "c".to_string()).unwrap();
    assert!(cache.get(&1).is_err());
    assert_eq!(cache.get(&2).unwrap(), "b");
    assert_eq!(cache.get(&3).unwrap(), "c");
    
    // 2. Проверка TTL: ждем 400мс. 2 и 3 должны истечь.
    thread::sleep(StdDuration::from_millis(400));
    assert!(cache.get(&2).is_err());
    assert!(cache.get(&3).is_err());
}