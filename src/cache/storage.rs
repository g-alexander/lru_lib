use std::collections::HashMap;
use std::hash::Hash;
use crate::error::Result;
use crate::models::CacheEntry;

/// In-memory storage for the cache.
/// Uses a `HashMap` for fast key-based access.
pub struct MemoryStorage<K, V>
where
    K: Hash + Eq + Clone,
{
    data: HashMap<K, CacheEntry<V>>,
}

impl<K, V> MemoryStorage<K, V>
where
    K: Hash + Eq + Clone,
{
    /// Создает новое пустое хранилище.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Вставляет элемент в хранилище.
    pub fn insert(&mut self, key: K, entry: CacheEntry<V>) -> Result<()> {
        self.data.insert(key, entry);
        Ok(())
    }

    /// Получает элемент из хранилища.
    pub fn get(&self, key: &K) -> Option<&CacheEntry<V>> {
        self.data.get(key)
    }

    /// Получает изменяемую ссылку на элемент.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut CacheEntry<V>> {
        self.data.get_mut(key)
    }

    /// Удаляет элемент из хранилища.
    pub fn remove(&mut self, key: &K) -> Option<CacheEntry<V>> {
        self.data.remove(key)
    }

    /// Проверяет наличие ключа в хранилище.
    pub fn contains(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    /// Очищает всё хранилище.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Возвращает количество элементов в хранилище.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Возвращает ссылку на внутреннюю карту данных.
    pub fn data_as_map(&self) -> &HashMap<K, CacheEntry<V>> {
        &self.data
    }
}

impl<K, V> Default for MemoryStorage<K, V>
where
    K: Hash + Eq + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}