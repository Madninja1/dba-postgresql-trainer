# Step 5. User-Defined Types

## 1. Topic

In Rust, the main tools for creating your own domain types are **structures (`struct`)** and **enumerations (`enum`)**.

```text
struct
→ one value consists of a set of related data

enum
→ a value can be one of several variants
```

Structures let you group related fields under one type. Enumerations let you describe a type as a set of allowed variants, and each variant can store its own data. Types such as `Option<T>` are built on these mechanisms, while `match` and `if let` are used to handle variants. The Rust Programming Language (2)

---

# 2. Mechanism

## Structures `struct`

A structure combines several related values into a new type.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

Each pair:

```text
name: type
```

is called a **field**.

Creating an instance:

```rust
let user = User {
    active: true,
    username: String::from("daniel"),
    email: String::from("mail@example.com"),
    sign_in_count: 1,
};
```

Unlike a tuple, structure fields have names, so their meaning does not depend on position. The Rust Programming Language (2)

Accessing a field:

```rust
user.email
```

---

## Mutable Structures

To change a field, the whole instance must be declared `mut`:

```rust
let mut user = User {
    active: true,
    username: String::from("daniel"),
    email: String::from("old@example.com"),
    sign_in_count: 1,
};

user.email = String::from("new@example.com");
```

You cannot declare only one field mutable while keeping the instance itself immutable.

```text
let mut user
      ↓
instance fields can be changed
```

The Rust Programming Language (2)

---

## Field Init Shorthand

If a variable name matches a field name:

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

instead of:

```rust
username: username,
email: email,
```

you can simply write:

```rust
username,
email,
```

This is only shorthand syntax; the meaning does not change. The Rust Programming Language (2)

---

## Structure Update Syntax `..`

You can create a new instance and take the remaining fields from an existing one:

```rust
let user2 = User {
    email: String::from("new@example.com"),
    ..user1
};
```

`..user1` must appear after the explicitly specified fields.

This mechanism is connected to the ownership rules already studied.

If a `String` field is transferred from `user1` into `user2`, that may be a **move**:

```text
user1.username: String
        ↓
      move
        ↓
user2.username
```

After this, using the whole `user1` may no longer be allowed.

If only `Copy` fields such as `bool` or `u64` are copied, those values are copied normally. The Rust Programming Language (2)

This connection is important:

```text
struct update syntax
        ↓
does not bypass ownership
        ↓
ordinary move/Copy rules apply to every field
```

---

## Tuple Structs

Structures may have unnamed fields:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```

Creating values:

```rust
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

Even though the internal field types are identical:

```text
Color ≠ Point
```

these are different user-defined types.

Therefore a function:

```rust
fn f(c: Color)
```

cannot accept a `Point`.

Field access resembles tuple access:

```rust
black.0
black.1
```

A tuple struct is useful when you need a **distinct type** but do not need names for each field. The Rust Programming Language (2)

---

## Unit-Like Structs

A structure can contain no fields at all:

```rust
struct AlwaysEqual;
```

Create an instance:

```rust
let subject = AlwaysEqual;
```

This is called a **unit-like struct** because it resembles the `()` type.

The book notes that such a structure is useful when you need to define behavior for a type but do not need to store any data inside it. The Rust Programming Language (2)

---

## Ownership of Structure Fields

The `User` example uses:

```rust
username: String,
email: String,
```

so the structure **owns this data itself**.

```text
User
 ├─ owns username
 └─ owns email
```

The data therefore exists as long as the structure exists.

A structure can also store references to data owned elsewhere, but then **lifetimes** must be described. The book postpones that mechanism until Chapter 10. The Rust Programming Language (2)

For now, use this practical model:

```text
field: String
→ the structure owns the string

field: &str
→ the structure borrows the string
→ lifetime rules are required
```

---

# Methods and `impl`

A structure's data can be given behavior.

For this, use an `impl` block:

```rust
impl Rectangle {
    ...
}
```

For example:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

Call it with:

```rust
rect.area()
```

A method resembles a function, but it is associated with a particular type and receives `self` as its first parameter. The Rust Programming Language (2)

---

## `self`, `&self`, `&mut self`

The book shows three ways a method can relate to an instance.

### `&self`

```rust
fn area(&self) -> u32
```

The method only reads the structure.

`&self` is shorthand for:

```rust
self: &Self
```

where `Self` means the type for which the `impl` block is written.

For:

```rust
impl Rectangle
```

`Self` means `Rectangle`.

---

### `&mut self`

If a method must modify the instance:

```rust
fn change(&mut self) {
    ...
}
```

this is the mutable borrowing mechanism already studied:

```text
&mut self
→ the method temporarily gets mutable access
→ ownership remains with the caller
```

---

### `self`

A method may accept:

```rust
fn method(self)
```

Then it takes ownership of the instance.

The book notes that this form is less common and is useful, for example, when a method transforms a value so that the original instance should no longer be used afterward.

Summary:

```text
&self
→ read

&mut self
→ modify

self
→ take ownership
```

---

## Method Arguments

After `self`, a method can accept ordinary parameters:

```rust
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width
            && self.height > other.height
    }
}
```

Call:

```rust
rect1.can_hold(&rect2)
```

`self` corresponds to `rect1`, while `other` corresponds to `rect2`.

---

## Automatic Referencing and Dereferencing in Method Calls

With:

```rust
object.method()
```

Rust automatically adds an appropriate form of:

```text
&
&mut
*
```

so that the receiver matches the method signature.

Therefore method calls usually do not require manually writing complicated combinations of dereferencing and borrowing.

Rust examines the first method parameter:

```text
&self     → read
&mut self → modify
self      → take ownership
```

and chooses the appropriate calling form.

---

# Associated Functions

All functions defined inside an `impl` block are called **associated functions**.

A method is an associated function that has a `self` parameter.

You can also define a function without `self`:

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

Call:

```rust
let square = Rectangle::square(10);
```

That is:

```text
instance.method()
→ instance method

Type::function()
→ associated function
```

Such functions are often used to create instances.

The book specifically notes that the name `new` is common for these functions but **is not a special Rust keyword**. The Rust Programming Language (2)

You may have several blocks:

```rust
impl Rectangle {
    fn area(&self) -> u32 { ... }
}

impl Rectangle {
    fn square(size: u32) -> Self { ... }
}
```

This is allowed and is equivalent to placing the methods in one suitable `impl` block.

---

# Debug Printing User-Defined Types

A structure does not automatically implement `Debug` formatting.

You can request it:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

After that, these are available:

```rust
println!("{rect:?}");
```

and the more expanded form:

```rust
println!("{rect:#?}");
```

The book also shows:

```rust
dbg!(&rect);
```

`dbg!` prints debugging information to `stderr`, including the file and line where it was called. `dbg!` itself returns the value passed to it, so if you do not want to transfer ownership, pass a reference. This is primarily a development convenience rather than a fundamental part of the type model.

---

# Enumerations `enum`

## Enum as a Set of Variants

An `enum` defines a type whose value can be **one of several variants**.

Simple example:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

Values:

```rust
IpAddrKind::V4
IpAddrKind::V6
```

Both belong to one type:

```text
IpAddrKind
```

Main idea:

```text
struct
→ contains a set of fields at the same time

enum
→ at a given moment is one variant
```

---

## Enum Variants Can Store Data

In Rust, each variant can have its own data shape:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

Here:

```text
Quit
→ no data

Move
→ named fields

Write
→ String

ChangeColor
→ three i32 values
```

All of them still have one type:

```text
Message
```

The Rust Programming Language (2)

This expresses the model:

```text
Message =
    Quit
    OR Move(x, y)
    OR Write(text)
    OR ChangeColor(r, g, b)
```

Instead of several separate structures, the variants share the common type `Message`.

---

## Methods for Enums

Just like a structure, an enumeration can have:

```rust
impl Message {
    fn call(&self) {
        ...
    }
}
```

and then:

```rust
message.call();
```

So `impl` is not limited to `struct`; enums can also have methods.

---

# `Option<T>`

One of the key standard enums is:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

It represents:

```text
a value exists
→ Some(T)

no value exists
→ None
```

`Option<T>`, `Some`, and `None` are in the prelude, so writing:

```rust
Option::Some(...)
```

is usually unnecessary.

Examples:

```rust
let number = Some(5);
let letter = Some('a');

let missing: Option<i32> = None;
```

In the last case, the type must be specified because from `None` alone the compiler cannot infer what type would appear inside `Some`. The Rust Programming Language (2)

---

## Why `Option<T>` Matters

`T` and `Option<T>` are **different types**.

For example:

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);
```

You cannot simply do:

```rust
x + y
```

because:

```text
i8 ≠ Option<i8>
```

Before using the inner `T`, the possibility of `None` must be handled.

This forces code to distinguish explicitly between:

```text
Some(value)
→ a value exists

None
→ no value exists
```

Thus absence is represented in the type system rather than hidden inside an ordinary value. The Rust Programming Language (2)

---

# `match`

`match` compares a value against **patterns**.

```rust
match value {
    pattern1 => expression1,
    pattern2 => expression2,
}
```

For example:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Rust checks the patterns in order and executes the code of the first matching arm. `match` itself is an expression, so the value of the executed arm becomes the value of the entire `match`. The Rust Programming Language (2)

---

## Extracting Data Through a Pattern

`match` can not only identify a variant but also extract the data stored inside it.

For example:

```rust
enum Coin {
    Penny,
    Quarter(UsState),
}
```

You can write:

```rust
match coin {
    Coin::Penny => 1,

    Coin::Quarter(state) => {
        println!("{state:?}");
        25
    }
}
```

When the value is:

```text
Coin::Quarter(UsState::Alaska)
```

the pattern:

```rust
Coin::Quarter(state)
```

binds:

```text
state = UsState::Alaska
```

So a pattern simultaneously:

```text
checks the shape of a value
+
extracts its internal data
```

---

## `Option<T>` + `match`

Typical handling:

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```

For:

```rust
Some(5)
```

the pattern:

```rust
Some(i)
```

extracts:

```text
i = 5
```

and produces:

```rust
Some(6)
```

For `None`, it returns `None`.

---

## `match` Must Be Exhaustive

One of its most important properties is:

> `match` must handle every possible case.

This does not compile:

```rust
match x {
    Some(i) => Some(i + 1),
}
```

if `x` has type `Option<i32>`.

The missing case is:

```rust
None
```

The compiler detects this.

For `Option<T>`, this is especially important:

```text
Some(T)
+
None
```

must be handled explicitly or covered by a catch-all pattern.

---

## Catch-All Pattern

You can handle specific cases and collect all remaining ones into one arm:

```rust
match dice_roll {
    3 => add_hat(),
    7 => remove_hat(),
    other => move_player(other),
}
```

`other` matches every remaining value and binds that value itself.

The catch-all arm must come after the specific ones because patterns are checked from top to bottom.

---

## `_` — Ignoring a Value

If the value of the remaining cases is not needed:

```rust
match dice_roll {
    3 => add_hat(),
    7 => remove_hat(),
    _ => reroll(),
}
```

`_`:

```text
matches any value
+
does not bind it to a variable
```

If nothing at all should happen for the remaining cases:

```rust
match value {
    special => do_something(),
    _ => (),
}
```

`()` represents no useful action/value.

---

# `if let`

Sometimes `match` is too verbose when only **one pattern** matters.

For example:

```rust
let config = Some(3u8);

match config {
    Some(max) => println!("{max}"),
    _ => (),
}
```

can be shortened to:

```rust
if let Some(max) = config {
    println!("{max}");
}
```

Model:

```text
if value matches pattern
        ↓
bind inner values
        ↓
execute the block
```

`if let` ignores the remaining variants. The Rust Programming Language (2)

Main tradeoff:

```text
match
→ more code
→ exhaustiveness checking for all variants

if let
→ shorter
→ other variants can simply be ignored
```

---

## `if let ... else`

You can add:

```rust
if let Some(value) = option {
    ...
} else {
    ...
}
```

The `else` roughly corresponds to the catch-all arm:

```rust
_ => ...
```

in a `match`.

---

# 3. Important Details

The main difference between `struct` and `enum`:

```text
struct User {
    name,
    email,
    active
}

one User value
contains all these fields at once
```

Whereas:

```text
enum Message {
    Quit,
    Write(String),
    Move { x, y }
}
```

means:

```text
a concrete Message
is exactly one variant
```

---

Typical modeling approach:

```text
"an object has several characteristics"
→ struct

"a value can be in one of several
states/forms"
→ enum
```

For behavior:

```text
impl Type {
    ...
}
```

For working with variants:

```text
match
if let
```

---

Remember the connection with ownership:

```text
struct {
    field: String
}

→ the structure owns the String
```

and:

```rust
let b = Struct {
    ..a
};
```

may move non-`Copy` fields out of `a`.

User-defined types do not bypass ownership rules — their fields follow the same `move`, `Copy`, and borrowing rules as ordinary values.

---

# 4. What to Remember

Minimal `struct` model:

```rust
struct User {
    name: String,
    active: bool,
}
```

Instance:

```rust
let user = User {
    name: String::from("Daniil"),
    active: true,
};
```

Access:

```rust
user.name
```

Mutation:

```rust
let mut user = ...;
user.active = false;
```

Shorthand:

```rust
User {
    name,
    active,
}
```

Update:

```rust
User {
    active: false,
    ..old_user
}
```

With `..old_user`, remember that non-`Copy` fields may be moved.

---

Methods:

```rust
impl Type {
    fn read(&self) { }

    fn change(&mut self) { }

    fn consume(self) { }
}
```

Remember:

```text
&self
→ read

&mut self
→ modify

self
→ take ownership
```

Associated function:

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        ...
    }
}
```

Call:

```rust
Rectangle::square(10)
```

Difference:

```text
object.method()
Type::function()
```

---

Enum:

```rust
enum Message {
    Quit,
    Write(String),
    Move { x: i32, y: i32 },
}
```

Each variant can contain its own data.

`Option<T>`:

```rust
Some(T)
None
```

and:

```text
T ≠ Option<T>
```

Therefore absence cannot be silently ignored.

---

`match`:

```rust
match value {
    PatternA => ...,
    PatternB(x) => ...,
}
```

Remember:

```text
match
→ matches by pattern
→ can extract inner data
→ is an expression
→ must cover every case
```

Catch-all patterns:

```rust
other => ...
```

when the value is needed,

or:

```rust
_ => ...
```

when the value is not needed.

---

`if let`:

```rust
if let Some(value) = option {
    ...
}
```

Use it when only one variant matters.

Main final model:

```text
STRUCT
→ what data makes up a value?

ENUM
→ which variant can the value be?

IMPL
→ what behavior belongs to the type?

MATCH
→ which variant was received and what data should be extracted?

OPTION<T>
→ does a value exist or not?
```

After this step, Rust can model a domain using more than primitive types: data can be grouped into `struct`s, states and alternatives can be represented with `enum`s, behavior can be attached through `impl`, and variants can be handled safely with `match`.

**Notes size:** approximately **10–12 thousand characters** including code.

**Time to copy into a notebook:** approximately **60–75 minutes**. Main priorities when writing: `struct`, `impl/self`, `enum`, `Option<T>`, exhaustive `match`, and the difference between `match` and `if let`.
