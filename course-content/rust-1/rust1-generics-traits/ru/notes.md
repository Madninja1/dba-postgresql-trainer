# Шаг 9. Generics и Traits

## 1. Тема

**Generics** позволяют писать один код для нескольких конкретных типов, заменяя конкретный тип параметром:

```text
i32 / f64 / String / ...
          ↓
          T
```

Мы уже использовали обобщённые стандартные типы:

```rust
Option<T>
Vec<T>
HashMap<K, V>
Result<T, E>
```

Главная цель generics — убрать дублирование, сохранив проверку типов на этапе компиляции. Язык программирования Rust(2)

Но одного `T` часто недостаточно. Например, функция может работать не с любым типом, а только с типами, которые:

```text
можно сравнивать
можно печатать
можно клонировать
имеют определённый метод
```

Для описания такого поведения существуют **traits**.

Общая модель:

```text
GENERICS
→ "тип пока неизвестен"

TRAITS
→ "но этот тип обязан уметь вот это"

TRAIT BOUNDS
→ ограничивают допустимые generic-типы
```

---

# 2. Механизм

## Generic-функции

Допустим, одинаковый алгоритм нужен для `i32` и `char`.

Вместо двух функций:

```rust
fn largest_i32(list: &[i32]) -> &i32 { ... }

fn largest_char(list: &[char]) -> &char { ... }
```

можно начать с generic-варианта:

```rust
fn largest<T>(list: &[T]) -> &T {
    ...
}
```

`T` объявляется:

```rust
<T>
```

между именем функции и параметрами.

Читать сигнатуру можно так:

```text
largest<T>
→ функция обобщена по типу T

list: &[T]
→ принимает срез элементов T

-> &T
→ возвращает ссылку на T
```

По соглашению параметры типов часто называют коротко:

```text
T
U
K
V
E
```

и пишут в `UpperCamelCase`. Язык программирования Rust(2)

---

## Generic не означает «можно делать что угодно»

Такой код:

```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

сам по себе не компилируется.

Причина:

```text
T
→ может быть вообще любым типом

но код требует:
item > largest
```

Компилятор не знает, поддерживает ли произвольный `T` операцию сравнения.

Поэтому нужны ограничения поведения:

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    ...
}
```

`PartialOrd` сообщает:

```text
T обязан поддерживать сравнение
```

Именно здесь generics соединяются с traits.

---

# Generic-структуры

Структура может содержать generic-поля:

```rust
struct Point<T> {
    x: T,
    y: T,
}
```

Теперь:

```rust
let p1 = Point { x: 5, y: 10 };
let p2 = Point { x: 1.0, y: 4.0 };
```

допустимы.

Но:

```rust
let p = Point {
    x: 5,
    y: 4.0,
};
```

не подходит этому определению.

Почему?

```text
x: T
y: T

→ оба поля должны иметь один конкретный тип
```

Если `x` определил `T` как integer, `y` тоже должен иметь этот тип. Язык программирования Rust(2)

---

## Несколько generic-параметров

Если поля могут быть разными:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}
```

Теперь допустимо:

```rust
let p = Point {
    x: 5,
    y: 4.0,
};
```

Здесь:

```text
T = integer
U = float
```

Обобщённых параметров может быть больше, но книга отдельно отмечает: большое их количество ухудшает читаемость и может означать, что код стоит разбить на части.

---

# Generic enum

Мы уже знаем основной пример:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

`Option<T>` выражает одну идею:

```text
значение T есть
или
значения нет
```

независимо от конкретного `T`.

---

Другой пример:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Где:

```text
T → успешное значение
E → ошибка
```

Например:

```text
Result<File, io::Error>
```

То есть generic enum позволяет переиспользовать **одну структуру вариантов** для большого количества конкретных типов.

---

# Generic-методы

Для:

```rust
struct Point<T> {
    x: T,
    y: T,
}
```

можно написать:

```rust
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

Важно наличие:

```rust
impl<T>
```

Так мы объявляем `T`, используемый затем в:

```rust
Point<T>
```

Метод существует для любого `Point<T>`.

---

## Метод только для конкретного типа

Можно реализовать поведение не для всех `Point<T>`, а только для определённой конкретизации:

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        ...
    }
}
```

Тогда:

```text
Point<f32>
→ метод есть

Point<i32>
→ такого метода нет
```

Generic-тип не обязан иметь одинаковые методы для всех возможных `T`.

---

## Generic самого метода

Метод тоже может объявлять собственные generic-параметры.

Книга показывает:

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(
        self,
        other: Point<X2, Y2>,
    ) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
```

Здесь:

```text
X1, Y1
→ generic-параметры самой структуры

X2, Y2
→ generic-параметры конкретного метода
```

Результат использует:

```text
x из self  → X1
y из other → Y2
```

Это показывает, что параметры структуры и параметры метода не обязаны совпадать.

---

# Мономорфизация

Важный вопрос: появляется ли runtime-цена за generics?

Книга отвечает — нет: Rust применяет **monomorphization**.

Допустим:

```rust
let integer = Some(5);
let float = Some(5.0);
```

Исходно существует:

```rust
Option<T>
```

Во время компиляции Rust создаёт специализированный код для реально использованных типов.

Концептуально:

```text
Option<T>
       ↓ compilation

Option<i32>
Option<f64>
```

То есть примерно как если бы программист вручную написал отдельные конкретные версии.

```text
generic source code
       ↓
compiler
       ↓
specialized concrete code
```

Поэтому использование generics не создаёт дополнительной проверки типа во время выполнения. Язык программирования Rust(2)

---

# Traits

## Что такое trait

Trait описывает **общее поведение**, которое тип обязан предоставлять.

Например:

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

Trait говорит:

```text
Тип, реализующий Summary,
обязан иметь:

summarize(&self) -> String
```

Сам trait может не определять конкретный тип данных — он определяет набор требуемых методов. Книга сравнивает traits с интерфейсами других языков, отмечая, что между ними есть различия. Язык программирования Rust(2)

---

## Реализация trait

Пусть:

```rust
struct NewsArticle {
    headline: String,
    author: String,
}
```

Реализация:

```rust
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}
```

Синтаксис:

```text
impl Trait for Type {
    ...
}
```

После этого:

```rust
article.summarize()
```

становится допустимым.

Другой тип может реализовать тот же trait совсем иначе:

```rust
impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

Получаем:

```text
NewsArticle ─┐
             ├─ Summary
SocialPost ──┘
```

Типы разные, но имеют общее гарантированное поведение.

---

## Trait должен быть в области видимости

Если trait определён в другом модуле или crate, для использования его методов его нужно подключить:

```rust
use aggregator::Summary;
```

Это продолжение уже изученной роли `use`.

---

# Orphan rule

Rust ограничивает, какие реализации trait можно добавлять.

Упрощённое правило из книги:

> Реализовать trait для типа можно, если trait или тип локален для текущего crate.

Например, в своём crate допустимо:

```text
внешний trait + наш type
→ ✓
```

Например:

```rust
impl Display for MyType
```

если `MyType` наш.

И:

```text
наш trait + внешний type
→ ✓
```

Например:

```rust
impl Summary for Vec<T>
```

если `Summary` определён нами.

Но:

```text
внешний trait + внешний type
→ ✗
```

Например нельзя в своём crate сделать:

```rust
impl Display for Vec<T>
```

если и `Display`, и `Vec` принадлежат стандартной библиотеке.

Причина — согласованность реализаций: иначе два независимых crate могли бы определить разные реализации одного trait для одного типа, и Rust не смог бы однозначно выбрать нужную. Книга связывает это ограничение с **coherence** и **orphan rule**. Язык программирования Rust(2)

---

# Реализация метода по умолчанию

Trait может предоставить готовую реализацию:

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

Тогда тип может просто написать:

```rust
impl Summary for NewsArticle {}
```

и получить `summarize()` автоматически.

---

## Переопределение default

Тип может заменить поведение:

```rust
impl Summary for SocialPost {
    fn summarize(&self) -> String {
        ...
    }
}
```

Получается:

```text
default implementation
        ↓
можно использовать как есть
        ↓
или переопределить
```

---

## Default-метод может использовать обязательный метод

Например:

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

Тип обязан реализовать:

```rust
summarize_author
```

а `summarize` получает готовое поведение.

Это позволяет trait задавать небольшой обязательный минимум, а поверх него предоставлять общую функциональность. Язык программирования Rust(2)

Ограничение книги: из переопределённой версии метода нельзя вызвать его же реализацию по умолчанию.

---

# Traits как параметры

## `impl Trait`

Если функция принимает любой тип, реализующий `Summary`:

```rust
fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}
```

Теперь функция принимает:

```text
NewsArticle ✓
SocialPost  ✓

String      ✗
i32         ✗
```

если последние не реализуют `Summary`.

Внутри функции разрешено использовать поведение, гарантированное trait.

---

# Trait bounds

`impl Trait` — сокращённая форма.

Это:

```rust
fn notify(item: &impl Summary)
```

эквивалентно в простом случае:

```rust
fn notify<T: Summary>(item: &T)
```

Запись:

```rust
T: Summary
```

называется **trait bound**.

Читать:

```text
T
→ произвольный тип,
но
T обязан реализовывать Summary
```

Язык программирования Rust(2)

---

## Когда `impl Trait` и `<T: Trait>` различаются по выразительности

Например:

```rust
fn notify(
    item1: &impl Summary,
    item2: &impl Summary,
)
```

параметры могут иметь разные конкретные типы:

```text
item1 = NewsArticle
item2 = SocialPost
```

Но:

```rust
fn notify<T: Summary>(
    item1: &T,
    item2: &T,
)
```

означает:

```text
item1 и item2
должны иметь один и тот же конкретный T
```

То есть generic-параметр позволяет выразить связь между типами нескольких аргументов.

---

# Несколько trait bounds

Тип может быть обязан реализовать несколько traits:

```rust
fn notify<T: Summary + Display>(item: &T) {
    ...
}
```

Здесь:

```text
T должен реализовать:
Summary
+
Display
```

То же возможно с `impl Trait`:

```rust
fn notify(item: &(impl Summary + Display)) {
    ...
}
```

---

# `where`

При большом количестве bounds сигнатура становится тяжёлой:

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(
    t: &T,
    u: &U,
) -> i32
```

Можно вынести ограничения:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    ...
}
```

`where` не меняет смысл.

Он только отделяет:

```text
структуру функции
от
ограничений типов
```

и улучшает читаемость сложных сигнатур.

---

# `impl Trait` в возвращаемом типе

Можно скрыть конкретный возвращаемый тип:

```rust
fn returns_summarizable() -> impl Summary {
    SocialPost {
        ...
    }
}
```

Вызывающий код знает:

```text
конкретный тип неизвестен

НО

он реализует Summary
```

Поэтому методы `Summary` использовать можно.

---

## Ограничение `-> impl Trait`

В материале книги важное ограничение:

```rust
fn make(switch: bool) -> impl Summary
```

не может в одной ветке возвращать:

```text
NewsArticle
```

а в другой:

```text
SocialPost
```

даже если оба реализуют `Summary`.

При:

```rust
-> impl Summary
```

функция должна возвращать **один конкретный тип**, реализующий trait.

```text
один скрытый concrete type
→ ✓

два разных concrete types
→ ✗
```

Решение для разных типов через trait objects книга откладывает до главы 17. Язык программирования Rust(2)

---

# Условная реализация методов

Traits позволяют добавлять методы generic-типу только тогда, когда `T` имеет необходимое поведение.

```rust
struct Pair<T> {
    x: T,
    y: T,
}
```

Для любого `T`:

```rust
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
```

Но сравнение и печать разрешим только при нужных traits:

```rust
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("{}", self.x);
        } else {
            println!("{}", self.y);
        }
    }
}
```

Получается:

```text
Pair<T>::new
→ для любого T

cmp_display
→ только если:
  T: Display + PartialOrd
```

Это называется условной реализацией методов. Язык программирования Rust(2)

---

# Blanket implementations

Можно реализовать trait сразу для **любого типа**, удовлетворяющего определённому bound.

Книга приводит концептуальный пример из стандартной библиотеки:

```rust
impl<T: Display> ToString for T {
    ...
}
```

То есть:

```text
любой T
который реализует Display
        ↓
получает ToString
```

Такая реализация называется **blanket implementation** — общей реализацией.

Поэтому, например:

```rust
let s = 3.to_string();
```

работает благодаря связи между реализованными traits. Язык программирования Rust(2)

---

# 3. Важные детали

## Generics и traits решают разные задачи

Не путать:

```text
<T>
→ тип абстрагирован

T: Trait
→ абстрагированный тип ограничен поведением
```

Например:

```rust
fn f<T>(x: T)
```

говорит только:

```text
есть некоторый T
```

А:

```rust
fn f<T: Display>(x: T)
```

дополнительно гарантирует:

```text
T можно форматировать через Display
```

---

## Trait bound определяет, что разрешено внутри функции

Если указано:

```rust
T: Summary
```

внутри функции можно рассчитывать на методы `Summary`.

Если нужен оператор сравнения:

```rust
T: PartialOrd
```

Если нужно форматирование:

```rust
T: Display
```

Компилятор проверяет соблюдение этих требований при компиляции, а не во время выполнения. Язык программирования Rust(2)

---

## Generics не означают динамическую типизацию

Например:

```rust
Vec<T>
```

не является контейнером «чего угодно».

Для конкретного экземпляра:

```text
T становится конкретным типом
```

и компилятор знает этот тип.

Мономорфизация превращает generic-код в специализированный код для использованных concrete types.

---

## Trait определяет контракт поведения

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

означает:

```text
"если тип заявляет Summary,
у него существует этот метод
с этой сигнатурой"
```

Это позволяет generic-функции использовать поведение, не зная конкретный тип.

---

## Ограничения реализаций нужны для согласованности

Запомнить orphan rule в практической форме:

```text
Trait наш?
или
Type наш?

хотя бы одно → реализация возможна

оба чужие → нельзя
```

Это предотвращает конфликтующие реализации из независимых crates.

---

# 4. Что запомнить

Минимальная generic-функция:

```rust
fn f<T>(value: T) {
    ...
}
```

Generic-структура:

```rust
struct Point<T> {
    x: T,
    y: T,
}
```

Разные типы полей:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}
```

Generic `impl`:

```rust
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

---

Trait:

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

Реализация:

```rust
impl Summary for MyType {
    fn summarize(&self) -> String {
        ...
    }
}
```

Главная схема:

```text
TRAIT
→ описывает поведение

IMPL TRAIT FOR TYPE
→ сообщает, как конкретный тип
  реализует это поведение
```

---

Trait bound:

```rust
fn f<T: Summary>(x: &T)
```

или:

```rust
fn f(x: &impl Summary)
```

Несколько требований:

```rust
T: Summary + Display
```

Сложные ограничения:

```rust
fn f<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
}
```

---

Важное различие:

```rust
fn f(a: &impl Summary, b: &impl Summary)
```

→ `a` и `b` могут быть разных типов.

```rust
fn f<T: Summary>(a: &T, b: &T)
```

→ `a` и `b` одного конкретного типа `T`.

---

Возврат:

```rust
fn f() -> impl Summary
```

означает:

```text
конкретный тип скрыт,
но он реализует Summary
```

При этом внутри одной такой функции возвращаемый concrete type должен оставаться одним и тем же.

---

Условная реализация:

```rust
impl<T: Display + PartialOrd> Pair<T> {
    ...
}
```

→ методы существуют только для подходящих `T`.

Blanket implementation:

```rust
impl<T: Display> ToString for T
```

→ trait реализуется сразу для всех `T`, удовлетворяющих bound.

---

Мономорфизация:

```text
generic code
    ↓ compile
concrete specialized code
```

Поэтому generics дают абстракцию без дополнительной runtime-платы, описанной в книге.

---

Итоговая модель темы:

```text
GENERICS
→ убирают дублирование типов

TRAITS
→ описывают общее поведение

TRAIT BOUNDS
→ говорят,
  какое поведение обязан иметь generic T

MONOMORPHIZATION
→ превращает generics
  в конкретный код при компиляции
```

Если запомнить одну сигнатуру, то эту:

```rust
fn operation<T: Trait>(value: &T)
```

Она читается:

```text
"operation работает с любым типом T,
если этот тип реализует Trait"
```

Это центральная связь между generics и traits.

**Объём конспекта:** примерно **10–12 тыс. символов** с кодом.

**Время переноса в тетрадь:** примерно **55–70 минут**. При сокращении можно убрать подробный `mixup`, но стоит обязательно оставить monomorphization, `impl Trait`, `<T: Trait>`, `+`, `where`, orphan rule, условные и blanket implementations.
