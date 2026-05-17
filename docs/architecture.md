# Архитектура библиотеки LRU Cache

## 1. Обзор системы
Библиотека построена на принципах модульности и разделения ответственности (Separation of Concerns). Основная задача — обеспечить эффективное управление элементами кэша с различными политиками вытеснения, сохраняя при этом потокобезопасность и возможность сохранения состояния.

## 2. Слои архитектуры

### 2.1. Layer: API (Public Interface)
Это верхний уровень, с которым взаимодействует пользователь. Он предоставляет высокоуровневые методы `get`, `put`, `remove`, `save`, `load`. Этот слой скрывает внутреннюю сложность реализации политик и механизмов синхронизации.

### 2.2. Layer: Core (Cache Engine)
Центральный компонент, реализующий логику управления кэшем.
- **Policy Manager:** Определяет, какой элемент подлежит удалению на основе выбранного режима (Capacity, TTL или Hybrid).
- **Storage Manager:** Управляет хранением данных в оперативной памяти.
- **Concurrency Controller:** Обеспечивает потокобезопасность через примитивы синхронизации (например, `RwLock`).

### 2.3. Layer: Infrastructure (Persistence & Storage)
Отвечает за взаимодействие с внешним миром.
- **Persistence Adapter:** Реализует интерфейс для сериализации и десериализации данных. Это позволяет легко менять формат хранения (JSON, Bincode) без изменения логики кэша.

## 3. Диаграмма компонентов (Mermaid)

```mermaid
componentDiagram
    package "User Space" {
        [Client Application]
    }

    package "LRU Cache Library" {
        [Cache API] --> [Cache Engine]
        [Cache Engine] --> [Policy Manager]
        [Cache Engine] --> [Memory Storage]
        [Cache Engine] --> [Persistence Adapter]
        
        interface "Persistence Trait" as P_Trait
        [Persistence Adapter] ..> P_Trait
    }

    [Client Application] --> [Cache API]
    [Persistence Adapter] --> [Disk/File System]
```

## 4. Дерево файлов

```text
rlu_lib/
├── src/
│   ├── lib.rs              # Точка входа, определение публичного API
│   ├── error.rs            # Определение типов ошибок библиотеки
│   ├── cache/              # Модуль ядра (Core)
│   │   ├── mod.rs         # Логика управления кэшем
│   │   ├── policy.rs       # Реализация политик (Capacity, TTL)
│   │   └── storage.rs      # Внутреннее хранилище (HashMap + List)
│   ├── persistence/        # Модуль инфраструктуры (Infrastructure)
│   │   ├── mod.rs           # Трейты и логика сохранения
│   │   └── json_adapter.rs # Реализация сохранения в JSON
│   └── models.rs           # Общие структуры данных (CacheEntry)
├── docs/
│   ├── architecture.md    # Данный файл
│   ├── requirements.md    # Требования
│   └── technical_spec.md    # Спецификация (будет создана аналитиком)
└── tests/                   # Интеграционные и стресс-тесты
```

## 5. Обоснование паттернов

| Паттерн | Применение | Преимущества |
| :--- | :--- | :--- |
| **Strategy** | Выбор политики вытеснения (Capacity vs TTL) | Позволяет легко добавлять новые политики без изменения основного движка кэша. |
| **Adapter** | Реализация персистентности | Отделяет логику кэша от конкретных форматов файлов (JSON, Bincode). |
| **Composition** | Комбинированный режим (Hybrid) | Режим Hybrid — это композиция стратегий Capacity и TTL. |

## 6. Связь с задачами бэклога
- **API Layer:** Задачи 001, 008.
- **Core Layer (Policy/Storage):** Задачи 002, 003, 004, 005.
- **Concurrency:** Задача 006.
- **Infrastructure Layer:** Задача 007.

---