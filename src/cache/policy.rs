use std::collections::VecDeque;
use std::hash::Hash;

/// Трейт для стратегий вытеснения элементов из кэша.
pub trait EvictionPolicy<K>
where
    K: Hash + Eq + Clone,
{
    /// Уведомляет политику о том, что к элементу обратились.
    fn on_access(&mut self, key: &K);

    /// Уведомляет политику о добавлении нового элемента.
    fn on_insert(&mut self, key: K);

    /// Возвращает ключ элемента, который должен быть удален согласно политике.
    fn next_eviction(&mut self) -> Option<K>;

    /// Удаляет ключ из политики (например, при явном удалении элемента из кэша).
    fn on_remove(&mut self, key: &K);
}

/// Реализация политики LRU (Least Recently Used) на основе VecDeque.
pub struct LruPolicy<K>
where
    K: Hash + Eq + Clone,
{
    // Хранит ключи в порядке их использования.
    // Первый элемент в очереди — самый старый.
    order: VecDeque<K>,
    // Для быстрого поиска позиции ключа в очереди (опционально, для оптимизации).
    // В данном простом варианте будем использовать VecDeque.
}

impl<K> LruPolicy<K>
where
    K: Hash + Eq + Clone,
{
    /// Создает новую политику LRU.
    pub fn new() -> Self {
        Self {
            order: VecDeque::new(),
        }
    }
}

impl<K> EvictionPolicy<K> for LruPolicy<K>
where
    K: Hash + Eq + Clone,
{
    fn on_access(&mut self, key: &K) {
        // Перемещаем ключ в конец очереди
        if let Some(pos) = self.order.iter().position(|x| x == key) {
            let k = self.order.remove(pos).unwrap();
            self.order.push_back(k);
        }
    }

    fn on_insert(&mut self, key: K) {
        self.order.push_back(key);
    }

    fn next_eviction(&mut self) -> Option<K> {
        // Извлекаем первый ключ (самый старый)
        self.order.pop_front()
    }

    fn on_remove(&mut self, key: &K) {
        if let Some(pos) = self.order.iter().position(|x| x == key) {
            self.order.remove(pos);
        }
    }
}
