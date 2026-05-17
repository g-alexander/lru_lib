# Задача 004: Реализация BinaryPersistenceAdapter

## 🎯 Цель
Реализовать новый адаптер персистентности, который сохраняет состояние кэша в бинарном формате с использованием библиотеки `bincode`, обеспечивая высокую скорость и компактность хранения.

## 📝 Описание
Необходимо создать новый модуль для бинарного адаптера и реализовать трейт `PersistenceAdapter`.

**Изменяемые файлы:**
- `src/persistence/binary_adapter.rs` (создание нового файла)
- `src/persistence/mod.rs` (регистрация модуля)

**Детали реализации:**
1. Создать файл `src/persistence/binary_adapter.rs`.
2. Реализовать структуру `BinaryPersistenceAdapter`.
3. Реализовать трейт `PersistenceAdapter<K, V>` для `BinaryPersistenceAdapter`:
    - `save`: использовать `bincode::serialize_into` для записи `HashMap<K, CacheEntry<V>>` в файл.
    - `load`: использовать `bincode::deserialize_from` для чтения данных из файла.
4. В `src/persistence/mod.rs` добавить `pub mod binary_adapter;` и `pub use binary_adapter::BinaryPersistenceAdapter;`.

## ✅ Критерии приемки (Acceptance Criteria)
- [ ] Адаптер `BinaryPersistenceAdapter` реализован.
- [ ] Адаптер успешно сохраняет и загружает данные в бинарном формате.
- [ ] Проект успешно компилируется.
- [ ] Данные, сохраненные бинарным адаптером, корректно восстанавливаются.

## 🛠 Шаги реализации
1. Создать файл `src/persistence/binary_adapter.rs` с реализацией трейта.
2. Подключить новый модуль в `src/persistence/mod.rs`.
3. Проверить компиляцию через `cargo check`.

## 🧪 Требования к юнит-тестам
- Создать тест, проверяющий цикл `save` -> `load` для `BinaryPersistenceAdapter`.
- Тест должен проверять целостность данных после бинарной сериализации/десериализации.
- Файл тестов: `tests/persistence_tests.rs`.

## 🔗 Зависимости
- Требует выполнения: `003_add_bincode_dependency.md`
- Блокирует: `005_register_binary_adapter.md`

---