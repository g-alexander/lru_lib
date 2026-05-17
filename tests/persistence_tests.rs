use lru_lib::cache::{Cache, LruCache};
use lru_lib::persistence::json_adapter::JsonPersistenceAdapter;
use lru_lib::persistence::BinaryPersistenceAdapter;
use std::path::PathBuf;
use chrono::Duration as ChronoDuration;

#[tokio::test]
async fn test_persistence_save_and_load() {
    let path = PathBuf::from("test_cache.json");
    let adapter = JsonPersistenceAdapter;
    let cache = LruCache::<i32, String>::new(Some(10), Some(ChronoDuration::minutes(5)));
    
    cache.put(1, "val1".to_string()).unwrap();
    cache.put(2, "val2".to_string()).unwrap();
    
    // Сохраняем
    cache.save_to_disk(&path, &adapter).await.unwrap();
    
    // Создаем новый кэш и загружаем данные
    let new_cache = LruCache::<i32, String>::new(Some(10), Some(ChronoDuration::minutes(5)));
    new_cache.load_from_disk(&path, &adapter).await.unwrap();
    
    // Проверяем данные
    assert_eq!(new_cache.get(&1).unwrap(), "val1");
    assert_eq!(new_cache.get(&2).unwrap(), "val2");
    
    // Удаляем временный файл
    let _ = std::fs::remove_file(path);
}

#[tokio::test]
async fn test_binary_persistence_save_and_load() {
    let path = PathBuf::from("test_cache.bin");
    let adapter = BinaryPersistenceAdapter;
    let cache = LruCache::<i32, String>::new(Some(10), Some(ChronoDuration::minutes(5)));
    
    cache.put(1, "bin1".to_string()).unwrap();
    cache.put(2, "bin2".to_string()).unwrap();
    
    // Сохраняем в бинарном формате
    cache.save_to_disk(&path, &adapter).await.unwrap();
    
    // Создаем новый кэш и загружаем данные
    let new_cache = LruCache::<i32, String>::new(Some(10), Some(ChronoDuration::minutes(5)));
    new_cache.load_from_disk(&path, &adapter).await.unwrap();
    
    // Проверяем данные
    assert_eq!(new_cache.get(&1).unwrap(), "bin1");
    assert_eq!(new_cache.get(&2).unwrap(), "bin2");
    assert_eq!(new_cache.len(), 2);
    
    // Удаляем временный файл
    let _ = std::fs::remove_file(path);
}