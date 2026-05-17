# Задача 005: Регистрация BinaryPersistenceAdapter

## 🎯 Цель
Сделать новый `BinaryPersistenceAdapter` доступным для использования через публичный интерфейс модуля `persistence`.

## 📝 Описание
Необходимо зарегистрировать новый модуль `binary_adapter` в `src/persistence/mod.rs` и добавить `BinaryPersistenceAdapter` в список публичных экспортов (`pub use`).

**Изменяемые файлы:**
- `src/persistence/mod.rs`

## ✅ Критерии приемки (Acceptance Criteria)
- [ ] Модуль `binary_adapter` объявлен как `pub mod`.
- [ ] Тип `BinaryPersistenceAdapter` доступен через `crate::persistence::BinaryPersistenceAdapter`.
- [ ] Проект успешно компилируется.

## 🛠 Шаги реализации
1. Открыть `src/persistence/mod.rs`.
2. Добавить `pub mod binary_adapter;`.
3. Добавить `pub use binary_adapter::BinaryPersistenceAdapter;`.
4. Запустить `cargo check`.

## 🧪 Требования к юнит-тестам
- Не требуется.

## 🔗 Зависимости
- Требует выполнения: `004_implement_binary_persistence_adapter.md`
- Блокирует: `006_add_persistence_tests.md`

---