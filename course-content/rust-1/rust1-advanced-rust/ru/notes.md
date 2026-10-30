# Шаг 15. Полная модель языка: trait objects, patterns, unsafe, macros

## 1. Тема

Финальный шаг объединяет возможности, которые нужны реже базового ownership-кода, но завершают общую модель Rust:

```text
trait objects
→ разные конкретные типы
  с общим поведением

patterns
→ разбор структуры значения

unsafe
→ операции, безопасность которых
  компилятор доказать не может

macros
→ генерация Rust-кода
  во время компиляции
```

К ним примыкают продвинутые traits и типы: associated types, fully qualified syntax, newtype, `Sized`, function pointers. Глава 19 сама подаёт эти возможности как инструменты для специфических ситуаций и как справочный материал, а не как ежедневный базовый синтаксис. Язык программирования Rust(2)

Общая модель после всей книги:

```text
ownership / borrowing
→ кто владеет данными и кто имеет доступ

types / generics / traits
→ какие значения допустимы и какое поведение гарантировано

patterns
→ как разобрать полученное значение

trait objects
→ как выбрать реализацию поведения runtime

unsafe
→ где программист вручную подтверждает инварианты

macros
→ как генерировать код compile time
```

---

## 2. Механизм

### Trait objects: `dyn Trait`

Generics с trait bounds работают, когда компилятор знает конкретный тип:

```rust
fn draw<T: Draw>(item: &T) {
    item.draw();
}
```

Для каждого реально используемого `T` происходит мономорфизация:

```text
T = Button
→ отдельный конкретный код

T = SelectBox
→ отдельный конкретный код
```

Это **static dispatch**: конкретный вызываемый метод известен во время компиляции.

Но иногда одна коллекция должна содержать разные типы:

```text
Button
SelectBox
TextField
...
```

если все они реализуют:

```rust
trait Draw {
    fn draw(&self);
}
```

Тогда используется trait object:

```rust
Box<dyn Draw>
```

Например:

```rust
struct Screen {
    components: Vec<Box<dyn Draw>>,
}
```

Теперь один `Vec` может содержать разные конкретные типы, если каждый реализует `Draw`. Язык программирования Rust(2)

```text
Vec<Box<dyn Draw>>
        │
        ├── Box<Button>
        ├── Box<SelectBox>
        └── Box<...>
```

Сам `Screen` знает только:

```text
у каждого элемента можно вызвать draw()
```

Конкретный тип ему знать не нужно.

---

### Почему нужен указатель

Trait object используется за некоторым указателем:

```rust
&dyn Draw
Box<dyn Draw>
```

Конкретный размер значения, скрытого за `dyn Draw`, заранее неизвестен, поэтому непосредственно хранить `dyn Draw` как обычное sized-значение нельзя.

Trait object связывает данные конкретного типа с информацией, необходимой для поиска реализации методов этого trait во время выполнения. Язык программирования Rust(2)

---

### Dynamic dispatch

При:

```rust
component.draw();
```

где:

```rust
component: Box<dyn Draw>
```

компилятор не знает заранее, находится внутри `Button` или другой тип.

Поэтому выбор реализации происходит во время выполнения:

```text
trait object
     ↓ runtime
определить concrete type / нужный метод
     ↓
вызвать соответствующий draw
```

Это **dynamic dispatch**.

В отличие от generics:

```text
<T: Draw>
→ static dispatch
→ monomorphization

dyn Draw
→ dynamic dispatch
→ выбор метода runtime
```

Динамическая диспетчеризация имеет runtime-затраты и ограничивает некоторые оптимизации, например inlining. Язык программирования Rust(2)

---

### Когда generics, когда trait objects

Если коллекция однородна:

```rust
struct Screen<T: Draw> {
    components: Vec<T>,
}
```

в одном `Screen<T>` все компоненты имеют один конкретный `T`.

Если нужна неоднородность:

```rust
Vec<Box<dyn Draw>>
```

можно хранить разные типы.

Итого:

```text
generic T
→ один concrete type для конкретного экземпляра
→ static dispatch

dyn Trait
→ разные concrete types
→ общее поведение
→ dynamic dispatch
```

---

### State pattern

Книга показывает trait objects на паттерне состояния.

Есть:

```text
Draft
→ PendingReview
→ Published
```

Каждое состояние реализует:

```rust
trait State
```

А `Post` хранит:

```rust
state: Option<Box<dyn State>>
```

Переход состояния выполняется через методы самого `State`, которые поглощают старое состояние и возвращают новое. Язык программирования Rust(2)

Концептуально:

```text
Post
 │
 └── Box<dyn State>
         │
         ├─ Draft
         ├─ PendingReview
         └─ Published
```

Поведение `Post` делегируется текущему объекту состояния.

Плюс:

```text
логика конкретного состояния
сосредоточена в одном типе
```

Минус — состояния оказываются связанными переходами, а часть логики может дублироваться. Язык программирования Rust(2)

---

### Состояние можно кодировать типами

Книга показывает альтернативу, лучше использующую систему типов Rust:

```text
DraftPost
→ PendingReviewPost
→ Post
```

У `DraftPost` можно вообще не определять метод:

```rust
content()
```

Тогда попытка прочитать неопубликованный пост становится не runtime-проверкой, а compile-time ошибкой.

```text
invalid state
→ невозможно выразить корректно типизированным кодом
```

Это важная общая идея Rust:

> если состояние должно быть запрещено, иногда лучше сделать его невозможным через систему типов, а не проверять `if` во время выполнения. Язык программирования Rust(2)

---

## Patterns

Pattern — описание **формы значения**.

Мы уже использовали их:

```rust
match value {
    Some(x) => ...
    None => ...
}
```

Pattern:

```rust
Some(x)
```

одновременно:

```text
проверяет:
value имеет вариант Some?

и

извлекает:
внутреннее значение → x
```

Но patterns используются не только в `match`.

---

### Где встречаются patterns

#### `match`

```rust
match value {
    Some(x) => ...,
    None => ...,
}
```

#### `if let`

```rust
if let Some(x) = value {
    ...
}
```

#### `while let`

```rust
while let Some(x) = stack.pop() {
    ...
}
```

Цикл продолжается, пока pattern совпадает. Язык программирования Rust(2)

#### `for`

Даже:

```rust
for x in values
```

содержит pattern `x`.

Можно сразу деструктурировать:

```rust
for (index, value) in items.iter().enumerate() {
    ...
}
```

#### `let`

```rust
let (x, y) = (1, 2);
```

#### параметры функций

```rust
fn print(&(x, y): &(i32, i32)) {
    ...
}
```

Patterns встроены глубже в язык, чем только `match`.

---

### Irrefutable и refutable patterns

**Irrefutable pattern** совпадает всегда.

```rust
let x = 5;
let (x, y) = (1, 2);
```

Такие patterns подходят для `let` и параметров функций.

**Refutable pattern** может не совпасть:

```rust
Some(x)
```

потому что значение может оказаться `None`.

Поэтому:

```rust
let Some(x) = value;
```

в общем случае не подходит как обычное безусловное связывание.

Для таких случаев нужны:

```text
match
if let
while let
```

Книга подчёркивает различие между refutable и irrefutable patterns как основу понимания того, где разрешён конкретный pattern. Язык программирования Rust(2)

---

### Деструктуризация

Структуры:

```rust
struct Point {
    x: i32,
    y: i32,
}

let Point { x, y } = point;
```

Enum:

```rust
match message {
    Message::Move { x, y } => ...
    Message::Write(text) => ...
    Message::Quit => ...
}
```

Tuple:

```rust
let (x, y, z) = tuple;
```

Можно вкладывать patterns друг в друга:

```rust
match value {
    Some(Point { x, y }) => ...
    None => ...
}
```

То есть pattern способен повторять структуру данных и одновременно извлекать нужные части.

---

### Несколько вариантов: `|`

```rust
match x {
    1 | 2 => println!("one or two"),
    _ => {}
}
```

Означает:

```text
pattern A
ИЛИ
pattern B
```

---

### Диапазоны

Для чисел и `char` можно сопоставлять диапазон:

```rust
match x {
    1..=5 => ...
    _ => ...
}
```

---

### Игнорирование `_`

```rust
match value {
    Some(_) => println!("есть значение"),
    None => {}
}
```

`_`:

```text
совпадает
+
не связывает значение
```

Это отличается от:

```rust
_unused
```

Имя, начинающееся с `_`, всё равно **является привязкой** и может получить ownership значения; оно только подавляет предупреждение о неиспользуемой переменной.

---

### `..`

Позволяет игнорировать оставшиеся части структуры:

```rust
let Point { x, .. } = point;
```

или:

```rust
let (first, .., last) = tuple;
```

Но `..` должен быть однозначным: компилятор должен понимать, какие именно значения он пропускает. Язык программирования Rust(2)

---

### Match guards

После pattern можно добавить дополнительное условие:

```rust
match num {
    Some(x) if x % 2 == 0 => ...
    Some(x) => ...
    None => ...
}
```

Pattern сначала должен совпасть, затем проверяется:

```rust
if condition
```

Это называется **match guard**. Язык программирования Rust(2)

Важно: guard является дополнительной логикой и не участвует в обычной проверке исчерпывающих patterns так же, как сами patterns.

---

### `@` binding

Иногда нужно одновременно:

```text
проверить форму значения
+
сохранить всё совпавшее значение
```

Для этого:

```rust
id @ 3..=7
```

Концептуально:

```text
3..=7
→ проверяет диапазон

id
→ получает совпавшее значение
```

`@` соединяет проверку pattern и привязку.

---

## Unsafe Rust

Safe Rust намеренно консервативен: если компилятор не может доказать безопасность операции, он может запретить даже код, который программист способен доказать корректным.

Для таких случаев есть:

```rust
unsafe {
    ...
}
```

Но `unsafe` **не отключает весь borrow checker и систему типов**.

Он разрешает только определённый набор операций, безопасность которых программист теперь обязан обеспечить самостоятельно. Язык программирования Rust(2)

---

### Пять возможностей `unsafe`

Книга перечисляет пять операций:

```text
1. Разыменовать raw pointer.

2. Вызвать unsafe function / method.

3. Читать или изменять mutable static.

4. Реализовать unsafe trait.

5. Получать доступ к полям union.
```

Язык программирования Rust(2)

Сам `unsafe` не означает, что код обязательно ошибочен. Он означает:

```text
compiler cannot verify this part
→ programmer guarantees invariants
```

---

### Raw pointers

Два вида:

```rust
*const T
*mut T
```

В отличие от обычных ссылок raw pointers:

```text
могут игнорировать обычные гарантии ссылок
могут быть null
могут указывать на недействительную память
не получают автоматических гарантий borrowing
```

Создать raw pointer можно и без `unsafe`, но **разыменовать** его можно только внутри `unsafe`.

```rust
unsafe {
    println!("{}", *ptr);
}
```

Именно операция доступа к памяти является опасной.

---

### Unsafe function

Функция может заявить:

```rust
unsafe fn dangerous() {
    ...
}
```

Её вызов требует:

```rust
unsafe {
    dangerous();
}
```

Смысл `unsafe fn`:

```text
у функции есть дополнительные условия,
которые компилятор проверить не способен
```

Вызывающий код обязан выполнить эти условия.

---

### Safe abstraction вокруг unsafe

Один из важнейших принципов главы:

```text
unsafe implementation
может иметь
safe public API
```

Книга показывает это на аналоге `split_at_mut`.

Borrow checker не способен самостоятельно доказать, что два возвращаемых `&mut` указывают на непересекающиеся части одного slice.

Реализация использует raw pointers и `unsafe`, но сначала проверяет границу `mid`, а наружу возвращает обычные безопасные ссылки.

```text
public safe function
        ↓
проверяет invariants
        ↓
маленький unsafe block
        ↓
возвращает safe abstraction
```

Такой подход ограничивает область, где программист вручную отвечает за безопасность. Язык программирования Rust(2)

---

### FFI и `extern`

Rust может взаимодействовать с функциями других языков через FFI.

Например, объявление функции с C ABI использует:

```rust
extern "C" {
    ...
}
```

`"C"` указывает ABI — соглашение о том, как функция вызывается на бинарном уровне.

Вызов внешней функции является unsafe, потому что Rust не может проверить гарантии кода, написанного вне Rust. Язык программирования Rust(2)

---

## Продвинутые traits и типы

Это вторичный материал шага, но несколько конструкций нужно узнавать.

### Associated types

Мы уже встречали:

```rust
trait Iterator {
    type Item;

    fn next(&mut self)
        -> Option<Self::Item>;
}
```

`Item` — associated type.

Реализация `Iterator` выбирает конкретный `Item` один раз:

```text
Iterator for Counter
→ Item = u32
```

В отличие от generic-параметра trait, associated type становится частью конкретного контракта реализации. Язык программирования Rust(2)

---

### Перегрузка операторов

Rust связывает операторы с traits.

Например:

```text
+
→ Add
```

Можно реализовать:

```rust
impl Add for Point {
    ...
}
```

и определить смысл:

```rust
point1 + point2
```

Создавать произвольные новые операторы нельзя; перегружаются поддерживаемые операции через соответствующие traits. Язык программирования Rust(2)

---

### Fully qualified syntax

Если несколько traits и сам тип имеют методы с одинаковым именем, можно явно указать нужную реализацию:

```rust
Pilot::fly(&person);
Wizard::fly(&person);
Human::fly(&person);
```

Для особенно неоднозначных associated functions используется полный синтаксис вида:

```rust
<Type as Trait>::function(...)
```

Он точно говорит компилятору:

```text
какой Trait
для какого Type
какую функцию
нужно вызвать
```

Язык программирования Rust(2)

---

### Newtype

Orphan rule не позволяет напрямую реализовать чужой trait для чужого типа.

Решение:

```rust
struct Wrapper(Vec<String>);
```

`Wrapper` — новый локальный тип.

Теперь для него можно написать:

```rust
impl Display for Wrapper {
    ...
}
```

Newtype:

```text
создаёт реально новый тип
→ даёт type safety
→ позволяет реализовывать traits
```

Книга отмечает, что такая оболочка устраняется при компиляции и сама по себе не добавляет runtime-стоимости. Язык программирования Rust(2)

Не путать с type alias:

```rust
type Kilometers = i32;
```

Alias — только другое имя того же типа:

```text
Kilometers == i32
```

Он не создаёт дополнительной type safety. Язык программирования Rust(2)

---

### DST и `Sized`

Некоторые типы не имеют известного compile-time размера:

```text
str
[T]
dyn Trait
```

Такие значения используются за указателями:

```rust
&str
&[T]
Box<dyn Trait>
```

Generic-параметр по умолчанию подразумевает:

```rust
T: Sized
```

Если нужно разрешить тип с неизвестным размером:

```rust
T: ?Sized
```

Но тогда с `T` обычно работают через ссылку или другой указатель. Язык программирования Rust(2)

---

### Function pointers

Обычная функция имеет тип:

```rust
fn
```

Например:

```rust
fn do_twice(
    f: fn(i32) -> i32,
    value: i32,
) -> i32 {
    f(value) + f(value)
}
```

Это именно **тип function pointer**, не trait `Fn`.

Function pointers реализуют `Fn`, `FnMut` и `FnOnce`, поэтому их можно передавать в места, ожидающие подходящее замыкание. Язык программирования Rust(2)

---

## Macros

Макрос — механизм **метапрограммирования**:

```text
Rust-код
→ macro
→ генерирует другой Rust-код
→ затем он компилируется
```

Мы уже использовали:

```rust
println!()
vec![]
panic!()
```

Макросы отличаются от функций тем, что работают с кодом на этапе компиляции. Они также могут принимать переменное количество аргументов и генерировать конструкции, которые функция runtime создать не может, например реализацию trait. Язык программирования Rust(2)

Цена — определения макросов сложнее читать и поддерживать.

---

### Declarative macros: `macro_rules!`

Базовая форма:

```rust
macro_rules! name {
    (pattern) => {
        generated_code
    };
}
```

По идее это напоминает `match`, только сопоставляется не runtime-значение, а структура исходного Rust-кода.

```text
input tokens
     ↓
pattern matching
     ↓
generated Rust code
```

Язык программирования Rust(2)

Упрощённая идея `vec!`:

```rust
macro_rules! vec {
    ($( $x:expr ),*) => {
        {
            let mut temp = Vec::new();

            $(
                temp.push($x);
            )*

            temp
        }
    };
}
```

Основные элементы:

```text
$x:expr
→ захватить Rust expression

$( ... ),*
→ повторить через запятую
  ноль или больше раз

$( ... )*
→ повторить сгенерированный код
```

Поэтому:

```rust
vec![1, 2, 3]
```

генерирует несколько `push`. Язык программирования Rust(2)

---

### Procedural macros

Вторая большая категория принимает исходный код как:

```rust
TokenStream
```

и возвращает новый:

```rust
TokenStream
```

```text
Rust tokens
    ↓
procedural macro
    ↓
Rust tokens
```

Язык программирования Rust(2)

Есть три вида:

```text
custom derive
attribute-like
function-like
```

---

### Custom derive

Пример использования:

```rust
#[derive(HelloMacro)]
struct Pancakes;
```

Макрос генерирует код реализации trait для типа.

Концептуально:

```text
struct Pancakes
        ↓ derive macro
генерируется:
impl HelloMacro for Pancakes { ... }
```

Для работы с кодом книга показывает `proc_macro`, `syn` и `quote`: `syn` разбирает Rust-код в структуру данных, а `quote!` помогает сгенерировать новый Rust-код. Язык программирования Rust(2)

---

### Attribute-like

Пример формы:

```rust
#[route(GET, "/")]
fn index() {
    ...
}
```

Такой макрос получает отдельно:

```text
содержимое attribute
+
аннотированный item
```

и генерирует новый код. Язык программирования Rust(2)

---

### Function-like procedural macro

Выглядит подобно обычному вызову макроса:

```rust
sql!(SELECT ...);
```

Но внутри может выполнять сложный разбор входного `TokenStream`, а затем вернуть сгенерированный Rust-код. Язык программирования Rust(2)

---

## 3. Важные детали

### Static и dynamic dispatch

Это одно из ключевых различий поздних глав:

```text
GENERICS

T: Trait
→ concrete type известен compile time
→ monomorphization
→ static dispatch
```

против:

```text
TRAIT OBJECT

dyn Trait
→ concrete type скрыт
→ runtime method selection
→ dynamic dispatch
```

Trait objects нужны не вместо generics вообще, а там, где runtime-полиморфизм действительно является частью модели.

---

### Patterns — это не просто `switch`

Pattern может:

```text
проверить variant
+
проверить literal/range
+
разобрать struct
+
извлечь поля
+
игнорировать части
+
создать bindings
```

Поэтому:

```rust
Some(Point { x, .. })
```

выражает существенно больше, чем обычное сравнение значения.

---

### Shadowing внутри pattern

Именованный pattern создаёт новую переменную.

Например, внешняя:

```rust
let y = 10;
```

и:

```rust
match x {
    Some(y) => ...
    _ => ...
}
```

Внутренний `y` — новая привязка, которая затеняет внешний `y`.

Если нужно сравнить со внешним значением, match guard может быть подходящим механизмом.

---

### `_` и `_name` различаются

```rust
_
```

не связывает значение.

```rust
_value
```

связывает его, просто подавляя предупреждение о неиспользовании.

Для ownership это может иметь значение: `_value` способен переместить значение, `_` — нет.

---

### `unsafe` лучше локализовать

Идеальная структура:

```text
большая safe codebase

        ↓

маленькая safe abstraction
        ↓
tiny unsafe implementation
```

Тогда проверять вручную нужно минимальный участок.

`unsafe` не должно использоваться для обхода неудобной ошибки компилятора без доказательства необходимых инвариантов.

---

### Memory safety не означает отсутствие всех ошибок

Даже safe Rust не исключает:

```text
ошибки бизнес-логики
deadlock
memory leak через Rc cycle
неправильные алгоритмы
```

`unsafe` относится прежде всего к гарантиям, которые компилятор не способен обеспечить для определённых низкоуровневых операций.

---

### Trait objects связаны с DST

```rust
dyn Trait
```

не имеет одного заранее известного размера, потому что concrete type может различаться.

Поэтому употребляется:

```rust
&dyn Trait
Box<dyn Trait>
```

Это соединяет главу 17 с `Sized/?Sized` из главы 19.

---

### Макросы стоит отличать от функций

```text
FUNCTION
→ получает runtime values
→ выполняется runtime

MACRO
→ получает syntax/tokens
→ генерирует Rust code
→ работает до обычного выполнения
```

Если задача решается обычной функцией, функция обычно проще. Макрос нужен, когда требуется работать со структурой кода, количеством аргументов или генерировать новые конструкции.

---

### Как глава 20 связывает всю книгу

Финальный web-server-проект не вводит фундаментально новую модель языка. Он соединяет изученное ранее:

```text
TcpListener / IO
→ работа с внешней системой

ThreadPool
→ structs + methods

Vec<Worker>
→ collections

thread::spawn
→ concurrency

FnOnce
→ closures

Send + 'static
→ traits + lifetimes + threads

channels
→ ownership transfer

Drop
→ graceful cleanup
```

Например, сигнатура `thread::spawn` снова связывает closure, `FnOnce`, `Send` и `'static`, а thread pool хранит `JoinHandle<()>` и передаёт работу потокам.

То есть глава 20 показывает не отдельный «web framework», а сборку ранее изученных механизмов в одно приложение.

---

## 4. Что запомнить

### Trait objects

```rust
Box<dyn Trait>
&dyn Trait
```

означают:

```text
concrete type неизвестен
но
Trait реализован
```

Главное отличие:

```text
<T: Trait>
→ static dispatch
→ один concrete type
→ monomorphization

dyn Trait
→ dynamic dispatch
→ разные concrete types
→ runtime choice
```

Типичный случай:

```rust
Vec<Box<dyn Draw>>
```

→ неоднородная коллекция с общим поведением.

---

### Patterns

Где используются:

```text
match
if let
while let
for
let
function parameters
```

Основные конструкции:

```rust
Some(x)
Point { x, y }
(a, b)
1 | 2
1..=5
_
..
pattern if condition
binding @ pattern
```

Запомнить:

```text
pattern
→ не только проверяет значение
→ но и деструктурирует его
→ и создаёт bindings
```

И различие:

```text
irrefutable
→ совпадает всегда
→ let / function params

refutable
→ может не совпасть
→ match / if let / while let
```

---

### Unsafe

```rust
unsafe {
    ...
}
```

не отключает Rust целиком.

Он разрешает пять специальных операций:

```text
raw pointer dereference
unsafe function/method call
mutable static access
unsafe trait implementation
union field access
```

Главная модель:

```text
unsafe
→ compiler больше не доказывает
  конкретный invariant

→ ответственность переходит программисту
```

Лучший архитектурный паттерн из главы:

```text
safe API
   ↓
small unsafe implementation
```

---

### Продвинутые конструкции

```text
Iterator::Item
→ associated type

<Type as Trait>::method
→ явный выбор реализации

struct Wrapper(T)
→ newtype

type Alias = ExistingType
→ только alias, не новый тип

T: ?Sized
→ T может иметь неизвестный compile-time размер

fn(i32) -> i32
→ function pointer
```

---

### Макросы

Два основных семейства:

```text
DECLARATIVE
→ macro_rules!
→ syntax pattern → generated code

PROCEDURAL
→ TokenStream → TokenStream
```

Procedural:

```text
#[derive(...)]
→ custom derive

#[attribute(...)]
→ attribute-like

name!(...)
→ function-like
```

Главное отличие от функции:

```text
function
→ работает со значениями

macro
→ работает с кодом
→ генерирует код compile time
```

---

### Финальная модель Rust

После всех 15 шагов язык можно свести к одной цепочке:

```text
VALUES
↓
имеют TYPES

TYPES
↓
имеют OWNERSHIP

OWNERSHIP
↓
контролирует lifetime данных

REFERENCES
↓
дают borrowing без передачи ownership

LIFETIMES
↓
доказывают корректность ссылок

GENERICS
↓
абстрагируют concrete types

TRAITS
↓
задают требуемое поведение

TRAIT OBJECTS
↓
дают runtime polymorphism

PATTERNS
↓
разбирают структуру значений

RESULT / OPTION
↓
явно выражают альтернативные состояния

SMART POINTERS
↓
расширяют модели ownership

SEND / SYNC + THREADS
↓
переносят ownership-модель
в многопоточность

UNSAFE
↓
локально передаёт часть доказательства
безопасности программисту

MACROS
↓
генерируют Rust-код compile time
```

Самая важная идея всей книги: **Rust старается как можно больше свойств программы представить через типы, владение, traits и compile-time проверки. Runtime-механизмы вроде trait objects, `RefCell`, `Mutex` или dynamic dispatch применяются там, где статической модели недостаточно или требуется дополнительная гибкость; `unsafe` используется ещё глубже — когда нужное свойство уже обязан доказать сам программист.**

**Объём конспекта:** примерно **14–16 тыс. символов** с кодом и схемами.

**Время переноса в тетрадь:** примерно **80–95 минут**. Если требуется уложиться ближе к часу, можно сократить state pattern, associated types/newtype/function pointers и детали procedural macros; ядро финального шага — `dyn Trait`, static vs dynamic dispatch, refutable/irrefutable patterns, деструктуризация, пять возможностей `unsafe`, safe abstraction around unsafe и отличие `macro_rules!` от procedural macros.
