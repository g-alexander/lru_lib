use lru_lib::cache::{Cache, LruCache};
use std::thread;
use std::time::Duration as StdDuration;
use chrono::Duration as ChronoDuration;

#[test]
fn test_ttl_expiration() {
    // Кэш с TTL 100мс
    let ttl = Some(ChronoDuration::milliseconds(100));
    let cache = LruCache::new(None, ttl);
    
    cache.put(1, "a".to_string()).unwrap();
    
    // Проверяем, что элемент на месте
    assert_eq!(cache.get(&1).unwrap(), "a");
    
    // Ждем чуть больше времени, чем TTL
    thread::sleep(StdDuration::from_millis(150));
    
    // Элемент должен быть удален
    assert!(cache.get(&1).is_err());
}

#[test]
fn test_ttl_refresh() {
    // Кэш с TTL 500мс
    let ttl = Some(ChronoDuration::milliseconds(500));
    let cache = LruCache::new(None, ttl);
    
    cache.put(1, "a".to_string()).unwrap();
    
    // Ждем 100мс
    thread::sleep(StdDuration::from_millis(100));
    
    // Обращаемся к элементу, чтобы сбросить счетчик (touch)
    assert_eq!(cache.get(&1).unwrap(), "a");
    
    // Ждем еще 100мс.
    thread::sleep(StdDuration::from_millis(100));
    assert_eq!(cache.get(&1).unwrap(), "a");
    
    // Ждем еще 400мс (всего 600мс с момента обращения, но TTL 500мс)
    // Мы используем 400мс, чтобы гарантировать, что 500мс точно истекли (с учетом погрешности сна)
    // На самом деле, нам нужно подождать БОЛЬШЕ 500мс.
    thread::sleep(StdDuration::from_millis(500));
    assert!(cache.get(&1).is_err());
}
