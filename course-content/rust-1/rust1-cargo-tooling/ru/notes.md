# Шаг 13. Cargo как рабочий инструмент

## 1. Тема

До этого Cargo использовался в основном для:

```text
cargo new
cargo check
cargo build
cargo run
cargo test
```

Глава 14 показывает Cargo уже как полноценный инструмент управления Rust-проектом:

```text
Cargo
├─ настраивает режимы компиляции
├─ генерирует документацию
├─ публикует crates
├─ управляет несколькими пакетами через workspace
├─ устанавливает готовые CLI-инструменты
└─ поддерживает пользовательские подкоманды
```

Основные темы главы: release profiles, публикация на crates.io, workspaces, `cargo install` и расширения Cargo. Язык программирования Rust(2)

---

# 2. Механизм

## Профили сборки

Cargo имеет разные **профили компиляции**.

Два основных:

```text
dev
→ cargo build
→ разработка

release
→ cargo build --release
→ релизная сборка
```

У них разные настройки компилятора.

Типичный компромисс:

```text
dev
→ быстрее компилировать
→ меньше оптимизаций

release
→ дольше компилировать
→ больше оптимизаций
→ быстрее итоговая программа
```

---

## Настройка профилей

Параметры задаются в `Cargo.toml`:

```toml
[profile.dev]
opt-level = 1
```

Если секции нет, Cargo использует значения по умолчанию.

В книге для `opt-level`:

```text
dev     → 0
release → 3
```

`opt-level` управляет уровнем оптимизаций компилятора в диапазоне `0..3`. Чем выше оптимизация, тем обычно больше времени требуется на компиляцию. Язык программирования Rust(2)

Главная модель:

```text
частая разработка
→ ценнее быстрая компиляция

готовый релиз
→ ценнее производительность программы
```

---

# Документация библиотеки

## `///` — документация элемента

Обычный комментарий:

```rust
// comment
```

Документационный:

```rust
/// Adds one to the number.
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

`///` применяется к элементу, который следует за ним:

```text
/// ...
↓
function / struct / enum / ...
```

Документация поддерживает Markdown. Язык программирования Rust(2)

---

## Примеры в документации

Можно писать:

```rust
/// Adds one.
///
/// # Examples
///
/// ```
/// let result = my_crate::add_one(5);
/// assert_eq!(result, 6);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

Генерация:

```bash
cargo doc
```

Результат помещается в:

```text
target/doc
```

Удобный вариант:

```bash
cargo doc --open
```

→ сгенерировать документацию и открыть её в браузере. Язык программирования Rust(2)

---

## Примеры документации являются тестами

Код внутри documentation comments проверяется командой:

```bash
cargo test
```

Получается:

```text
документация
    ↓
пример кода
    ↓
doc-test
    ↓
Cargo проверяет,
что пример продолжает работать
```

Это защищает от ситуации:

```text
API изменился
+
пример остался старым
```

Язык программирования Rust(2)

---

## Полезные разделы документации

Книга отдельно выделяет:

```text
# Examples
→ пример использования

# Panics
→ когда функция может panic

# Errors
→ какие Err возможны и почему

# Safety
→ требования unsafe-функции
```

Не каждая функция требует все разделы; это ориентиры для документирования важных свойств API. Язык программирования Rust(2)

---

## `//!` — документация контейнера

`///` документирует **следующий элемент**.

`//!` документирует **элемент, внутри которого находится комментарий**.

Например, в начале `src/lib.rs`:

```rust
//! # My Crate
//!
//! Utilities for ...
```

описание относится ко всему crate.

То же можно использовать внутри модуля для его общего описания. Язык программирования Rust(2)

Запомнить:

```text
/// item
→ документация следующего элемента

//! module/crate
→ документация текущего контейнера
```

---

# Публичный API и `pub use`

Внутренняя структура библиотеки не обязана совпадать с тем, что видит пользователь.

Например, внутри удобно:

```text
crate
├─ kinds
│  └─ PrimaryColor
└─ utils
   └─ mix
```

Без реэкспорта пользователь пишет:

```rust
use art::kinds::PrimaryColor;
use art::utils::mix;
```

Можно предоставить более простой API:

```rust
pub use self::kinds::PrimaryColor;
pub use self::utils::mix;
```

Теперь:

```rust
use art::PrimaryColor;
use art::mix;
```

Язык программирования Rust(2)

Главная идея:

```text
внутренняя архитектура
≠ обязательно публичный API
```

`pub use` позволяет менять представление библиотеки пользователю без необходимости полностью перестраивать внутренние модули.

---

# Публикация crate

Для публикации книга использует:

```bash
cargo publish
```

До публикации `Cargo.toml` должен содержать необходимые метаданные.

Пример:

```toml
[package]
name = "my_crate"
version = "0.1.0"
edition = "2021"
description = "Description of the crate."
license = "MIT"
```

Имя на crates.io должно быть уникальным. Книга также требует для публикации описание и лицензию; вместо идентификатора лицензии можно использовать `license-file`, если лицензия хранится отдельным файлом. Язык программирования Rust(2)

---

## Версии после публикации

Опубликованная версия не перезаписывается.

Для выпуска новой:

```text
1. изменить version в Cargo.toml
2. выбрать номер по Semantic Versioning
3. снова cargo publish
```

Книга подчёркивает, что публикация версии является постоянной: уже загруженную версию нельзя просто заменить новым содержимым. Язык программирования Rust(2)

---

## `cargo yank`

Если опубликованная версия оказалась проблемной:

```bash
cargo yank --vers 1.0.1
```

Это **не удаляет** версию.

Механизм:

```text
существующие проекты
с этой версией в Cargo.lock
→ продолжают работать

новые разрешения зависимостей
→ больше не выбирают эту версию
```

Отменить:

```bash
cargo yank --vers 1.0.1 --undo
```

Важно:

```text
yank ≠ delete
```

Если в опубликованный код случайно попал секрет, `yank` его не удалит. Книга указывает в таком случае немедленно заменить скомпрометированный секрет. Язык программирования Rust(2)

---

# Workspaces

## Что такое workspace

**Workspace** объединяет несколько взаимосвязанных Cargo-пакетов, которые разрабатываются вместе.

Главные общие ресурсы:

```text
один Cargo.lock
+
один общий target/
```

Язык программирования Rust(2)

Пример:

```text
add/
├─ Cargo.toml
├─ Cargo.lock
├─ target/
├─ adder/
│  ├─ Cargo.toml
│  └─ src/main.rs
└─ add_one/
   ├─ Cargo.toml
   └─ src/lib.rs
```

Корневой `Cargo.toml` описывает workspace:

```toml
[workspace]
resolver = "3"
members = [
    "adder",
    "add_one",
]
```

В таком корневом manifest нет обычного `[package]`: он описывает рабочее пространство.

---

## Зачем общий `target`

Даже если собрать отдельный пакет внутри workspace:

```bash
cargo build
```

артефакты размещаются в общем:

```text
workspace/target/
```

Это позволяет нескольким пакетам совместно использовать результаты компиляции и уменьшает ненужную перекомпиляцию. Язык программирования Rust(2)

---

## Зависимости между участниками workspace

То, что два пакета находятся в одном workspace, **не делает их автоматически зависимыми друг от друга**.

Если `adder` использует `add_one`, это нужно явно указать:

```toml
[dependencies]
add_one = { path = "../add_one" }
```

После этого:

```text
adder
  ↓ depends on
add_one
```

Язык программирования Rust(2)

---

## Внешние зависимости workspace

Общий `Cargo.lock` не означает, что внешняя зависимость автоматически доступна всем пакетам.

Если `add_one` использует `rand`:

```toml
[dependencies]
rand = "..."
```

это не позволяет сразу написать:

```rust
use rand;
```

в `adder`.

`adder/Cargo.toml` тоже должен объявить `rand` своей зависимостью.

Итого:

```text
Cargo.lock
→ общий выбор версий

Cargo.toml каждого package
→ собственный список разрешённых зависимостей
```

Язык программирования Rust(2)

---

## Команды workspace

Сборка из корня:

```bash
cargo build
```

Работа с конкретным пакетом:

```bash
cargo run -p adder
```

Тесты всего workspace:

```bash
cargo test
```

Тесты одного пакета:

```bash
cargo test -p add_one
```

Если crates из workspace публикуются, каждый из них публикуется отдельно. Язык программирования Rust(2)

---

# `cargo install`

Cargo умеет устанавливать исполняемые Rust-пакеты:

```bash
cargo install package_name
```

Например, книга показывает:

```bash
cargo install ripgrep
```

Устанавливать таким способом можно пакеты, содержащие **binary target**, то есть программу, которую можно непосредственно запустить. `cargo install` не предназначен для установки обычных библиотечных зависимостей проекта. Язык программирования Rust(2)

---

## Где находятся программы

При типичной установке Rust через `rustup` бинарные файлы попадают в:

```text
$HOME/.cargo/bin
```

Если каталог находится в `$PATH`, установленную программу можно запускать просто по имени:

```text
cargo install ...
        ↓
~/.cargo/bin/tool
        ↓
tool
```

---

# Пользовательские команды Cargo

Cargo можно расширять без изменения самого Cargo.

Если в `$PATH` есть программа:

```text
cargo-something
```

то её можно запускать:

```bash
cargo something
```

Например:

```text
cargo-abc
   ↓
cargo abc
```

Пользовательские подкоманды можно увидеть вместе с другими командами через:

```bash
cargo --list
```

Таким образом внешний инструмент может выглядеть как естественное расширение Cargo. Язык программирования Rust(2)

---

# 3. Важные детали

Cargo здесь работает на нескольких уровнях:

```text
COMPILATION
→ profiles

LIBRARY API
→ cargo doc
→ ///, //!
→ pub use

DISTRIBUTION
→ cargo publish
→ cargo yank

LARGE PROJECT
→ workspace

TOOLS
→ cargo install
→ custom subcommands
```

Самые важные различия:

```text
cargo build
→ dev profile

cargo build --release
→ release profile
```

и:

```text
Cargo.toml package
→ описывает один пакет

Cargo.toml workspace
→ организует несколько пакетов
```

---

Не путать `workspace` и зависимости:

```text
same workspace
≠ автоматически можно use другой crate
```

Связь всё равно нужно объявить:

```toml
[dependencies]
some_crate = { path = "../some_crate" }
```

---

Не путать общий `Cargo.lock` с общими зависимостями:

```text
Cargo.lock
→ фиксирует совместно выбранные версии

[dependencies]
→ определяет, чем конкретно может пользоваться package
```

---

Документация — часть рабочего процесса:

```text
/// API docs
      ↓
cargo doc
      ↓
HTML

code examples
      ↓
cargo test
      ↓
doc-tests
```

Поэтому пример в документации одновременно объясняет API и проверяется на актуальность.

---

# 4. Что запомнить

Профили:

```bash
cargo build
```

→ `dev`.

```bash
cargo build --release
```

→ `release`.

Настройка:

```toml
[profile.dev]
opt-level = 1
```

---

Документация:

```rust
/// Documentation for next item.

//! Documentation for current module/crate.
```

Команды:

```bash
cargo doc
cargo doc --open
cargo test
```

Кодовые примеры из документации участвуют в doc-tests.

---

Удобный публичный API:

```rust
pub use crate::internal::SomeType;
```

Запомнить:

```text
pub use
→ реэкспорт
→ внутренние модули можно скрыть
  за более удобным публичным API
```

---

Публикация:

```bash
cargo publish
```

Новая версия:

```text
изменить version
→ cargo publish
```

Проблемную версию:

```bash
cargo yank --vers 1.0.1
```

Вернуть:

```bash
cargo yank --vers 1.0.1 --undo
```

```text
yank
≠ удаление версии
```

---

Workspace:

```toml
[workspace]
members = [
    "app",
    "library",
]
```

Главная модель:

```text
WORKSPACE
├─ общий Cargo.lock
├─ общий target/
├─ package A
├─ package B
└─ package C
```

Но:

```text
A использует B
→ зависимость должна быть указана явно
```

Например:

```toml
[dependencies]
library = { path = "../library" }
```

---

Основные команды workspace:

```bash
cargo build
cargo test

cargo run -p package
cargo test -p package
```

---

Установка CLI-инструмента:

```bash
cargo install crate_name
```

Типичное место:

```text
$HOME/.cargo/bin
```

Расширение Cargo:

```text
cargo-tool
→ cargo tool
```

Проверить доступные команды:

```bash
cargo --list
```

---

Итоговая схема:

```text
DEVELOP
→ cargo build
→ dev profile

RELEASE
→ cargo build --release
→ release profile

DOCUMENT
→ cargo doc
→ doc-tests

ORGANIZE
→ workspace

PUBLISH
→ cargo publish
→ cargo yank

INSTALL TOOLS
→ cargo install

EXTEND
→ cargo-something
→ cargo something
```

Главная мысль: **Cargo — не просто команда сборки. Он связывает жизненный цикл Rust-проекта: разработку, конфигурацию компиляции, документацию, тестирование, организацию нескольких пакетов, публикацию и установку инструментов.**

**Объём конспекта:** примерно **8–10 тыс. символов** с кодом.

**Время переноса в тетрадь:** примерно **40–55 минут**. Для долгосрочной памяти достаточно особенно хорошо закрепить profiles, `cargo doc`, `pub use`, модель workspace, `cargo publish/yank` и `cargo install`.
