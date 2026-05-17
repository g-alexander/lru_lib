use lru_lib::cache::{Cache, LruCache};

#[test]
fn test_capacity_limit() {
    let cache = LruCache::new(Some(2), None);
    
    cache.put(1, "a".to_string()).unwrap();
    cache.put(2, "b".to_string()).unwrap();
    
    // Проверяем, что оба элемента на месте
    assert_eq!(cache.get(&1).unwrap(), "a");
    assert_eq!(cache.get(&2).unwrap(), "b");
    
    // Добавляем третий элемент, первый (1) должен быть удален
    cache.put(3, "c".to_string()).unwrap();
    
    assert!(cache.get(&1).is_err());
    assert_eq!(cache.get(&2).unwrap(), "b");
    assert_eq!(cache.get(&3).unwrap(), "c");
}

#[test]
fn test_lru_behavior() {
    let cache = LruCache::new(Some(2), None);
    
    cache.put(1, "a".to_string()).unwrap();
    cache.put(2, "b".to_string()).unwrap();
    
    // Обращаемся к 1, чтобы сделать его "свежим"
    cache.get(&1).unwrap();
    
    // Добавляем 3, теперь должен удалиться 2, так как 1 был недавно использован
    cache.put(3, "c".to_string()).unwrap();
    
    assert_eq!(cache.get(&1).unwrap(), "a");
    assert!(cache.get(&2).is_err());
    assert_eq!(cache.get(&3).unwrap(), "c");
}

#[test]
fn test_remove_and_clear() {
    let cache = LruCache::new(Some(2), None);
    cache.put(1, "a".to_string()).unwrap();
    cache.put(2, "b".to_string()).unwrap();
    
    cache.remove(&1).unwrap();
    assert!(cache.get(&1).is_err());
    assert_eq!(cache.get(&2).unwrap(), "b");
    
    cache.clear().unwrap();
    assert!(cache.get(&2).is_err());
}