# Step 9. Generics and Traits

## 1. Topic

**Generics** let you write one implementation for several concrete types by replacing a concrete type with a parameter:

```text
i32 / f64 / String / ...
          ↓
          T
```

We have already used generic standard-library types:

```rust
Option<T>
Vec<T>
HashMap<K, V>
Result<T, E>
```

The main purpose of generics is to remove duplication while preserving compile-time type checking. The Rust Programming Language (2)

But a bare `T` is often not enough. A function may not work with every possible type, but only with types that:

```text
can be compared
can be printed
can be cloned
provide a particular method
```

**Traits** describe this kind of behavior.

General model:

```text
GENERICS
→ "the type is not known yet"

TRAITS
→ "but this type must support this behavior"

TRAIT BOUNDS
→ restrict the allowed generic types
```

---

# 2. Mechanism

## Generic Functions

Suppose the same algorithm is needed for `i32` and `char`.

Instead of two functions:

```rust
fn largest_i32(list: &[i32]) -> &i32 { ... }

fn largest_char(list: &[char]) -> &char { ... }
```

you can start with a generic version:

```rust
fn largest<T>(list: &[T]) -> &T {
    ...
}
```

`T` is declared as:

```rust
<T>
```

between the function name and its parameters.

Read the signature like this:

```text
largest<T>
→ function is generic over type T

list: &[T]
→ accepts a slice of T values

-> &T
→ returns a reference to T
```

By convention, type parameters are often short names such as:

```text
T
U
K
V
E
```

and are written in `UpperCamelCase`. The Rust Programming Language (2)

---

## Generic Does Not Mean “Anything Is Allowed”

This code:

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

does not compile by itself.

Reason:

```text
T
→ can be any type

but the code requires:
item > largest
```

The compiler does not know whether an arbitrary `T` supports comparison.

So behavioral constraints are required:

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    ...
}
```

`PartialOrd` says:

```text
T must support comparison
```

This is where generics connect with traits.

---

# Generic Structures

A structure can contain generic fields:

```rust
struct Point<T> {
    x: T,
    y: T,
}
```

Now:

```rust
let p1 = Point { x: 5, y: 10 };
let p2 = Point { x: 1.0, y: 4.0 };
```

are valid.

But:

```rust
let p = Point {
    x: 5,
    y: 4.0,
};
```

does not fit this definition.

Why?

```text
x: T
y: T

→ both fields must have the same concrete type
```

If `x` determines `T` to be an integer type, `y` must have that same type. The Rust Programming Language (2)

---

## Multiple Generic Parameters

If the fields may have different types:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}
```

then this is valid:

```rust
let p = Point {
    x: 5,
    y: 4.0,
};
```

Here:

```text
T = integer
U = float
```

There can be more generic parameters, but the book notes that too many reduce readability and may indicate that the code should be split into smaller pieces.

---

# Generic Enum

We already know the main example:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

`Option<T>` expresses one idea:

```text
a T value exists
or
no value exists
```

regardless of the concrete `T`.

---

Another example:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Where:

```text
T → successful value
E → error
```

For example:

```text
Result<File, io::Error>
```

So a generic enum lets one **variant structure** be reused with many concrete types.

---

# Generic Methods

For:

```rust
struct Point<T> {
    x: T,
    y: T,
}
```

you can write:

```rust
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

The important part is:

```rust
impl<T>
```

which declares the `T` later used in:

```rust
Point<T>
```

The method exists for any `Point<T>`.

---

## Method Only for a Specific Type

Behavior can be implemented not for every `Point<T>`, but only for a particular specialization:

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        ...
    }
}
```

Then:

```text
Point<f32>
→ method exists

Point<i32>
→ method does not exist
```

A generic type does not have to expose the same methods for every possible `T`.

---

## A Method's Own Generic Parameters

A method can declare its own generic parameters.

The book shows:

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

Here:

```text
X1, Y1
→ generic parameters of the structure

X2, Y2
→ generic parameters of this method
```

The result uses:

```text
x from self  → X1
y from other → Y2
```

This shows that a structure's parameters and a method's own parameters do not have to be the same.

---

# Monomorphization

An important question: do generics have a runtime cost?

The book's answer is no: Rust uses **monomorphization**.

Suppose:

```rust
let integer = Some(5);
let float = Some(5.0);
```

The source type is:

```rust
Option<T>
```

During compilation Rust creates specialized code for the concrete types that are actually used.

Conceptually:

```text
Option<T>
       ↓ compilation

Option<i32>
Option<f64>
```

This is roughly as if the programmer had manually written separate concrete versions.

```text
generic source code
       ↓
compiler
       ↓
specialized concrete code
```

So generics do not add runtime type checks. The Rust Programming Language (2)

---

# Traits

## What Is a Trait?

A trait describes **shared behavior** that a type must provide.

For example:

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

The trait says:

```text
A type implementing Summary
must provide:

summarize(&self) -> String
```

The trait does not have to define a concrete data type — it defines a set of required methods. The book compares traits with interfaces in other languages while noting that they are not identical. The Rust Programming Language (2)

---

## Implementing a Trait

Suppose:

```rust
struct NewsArticle {
    headline: String,
    author: String,
}
```

Implementation:

```rust
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}
```

Syntax:

```text
impl Trait for Type {
    ...
}
```

Afterward:

```rust
article.summarize()
```

is valid.

Another type can implement the same trait differently:

```rust
impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

So:

```text
NewsArticle ─┐
             ├─ Summary
SocialPost ──┘
```

The types differ, but they share guaranteed behavior.

---

## The Trait Must Be in Scope

If a trait is defined in another module or crate, it must be brought into scope to use its methods:

```rust
use aggregator::Summary;
```

This continues the already studied role of `use`.

---

# Orphan Rule

Rust restricts which trait implementations can be added.

Simplified rule from the book:

> You can implement a trait for a type if either the trait or the type is local to the current crate.

For example, in your own crate this is allowed:

```text
external trait + our type
→ ✓
```

For example:

```rust
impl Display for MyType
```

if `MyType` is ours.

And:

```text
our trait + external type
→ ✓
```

For example:

```rust
impl Summary for Vec<T>
```

if `Summary` is defined by us.

But:

```text
external trait + external type
→ ✗
```

For example, your crate cannot define:

```rust
impl Display for Vec<T>
```

when both `Display` and `Vec` belong to the standard library.

The reason is implementation coherence: otherwise two independent crates could provide different implementations of the same trait for the same type, and Rust could not choose unambiguously. The book connects this restriction with **coherence** and the **orphan rule**. The Rust Programming Language (2)

---

# Default Method Implementations

A trait can provide a ready-made implementation:

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

Then a type can simply write:

```rust
impl Summary for NewsArticle {}
```

and receive `summarize()` automatically.

---

## Overriding the Default

A type can replace the behavior:

```rust
impl Summary for SocialPost {
    fn summarize(&self) -> String {
        ...
    }
}
```

So:

```text
default implementation
        ↓
can be used as-is
        ↓
or overridden
```

---

## A Default Method Can Use a Required Method

For example:

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

A type must implement:

```rust
summarize_author
```

while `summarize` gets shared behavior for free.

This lets a trait define a small required core and build common functionality on top of it. The Rust Programming Language (2)

A limitation noted by the book: an overridden method cannot call the trait's default implementation of that same method.

---

# Traits as Parameters

## `impl Trait`

If a function accepts any type implementing `Summary`:

```rust
fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}
```

then it accepts:

```text
NewsArticle ✓
SocialPost  ✓

String      ✗
i32         ✗
```

if the latter types do not implement `Summary`.

Inside the function, you may use behavior guaranteed by the trait.

---

# Trait Bounds

`impl Trait` is shorthand.

This:

```rust
fn notify(item: &impl Summary)
```

is equivalent in a simple case to:

```rust
fn notify<T: Summary>(item: &T)
```

The notation:

```rust
T: Summary
```

is called a **trait bound**.

Read it as:

```text
T
→ any type,
but
T must implement Summary
```

The Rust Programming Language (2)

---

## Where `impl Trait` and `<T: Trait>` Differ in Expressiveness

For example:

```rust
fn notify(
    item1: &impl Summary,
    item2: &impl Summary,
)
```

the parameters may have different concrete types:

```text
item1 = NewsArticle
item2 = SocialPost
```

But:

```rust
fn notify<T: Summary>(
    item1: &T,
    item2: &T,
)
```

means:

```text
item1 and item2
must have the same concrete T
```

So a named generic parameter can express a relationship between the types of several arguments.

---

# Multiple Trait Bounds

A type may be required to implement several traits:

```rust
fn notify<T: Summary + Display>(item: &T) {
    ...
}
```

Here:

```text
T must implement:
Summary
+
Display
```

The same is possible with `impl Trait`:

```rust
fn notify(item: &(impl Summary + Display)) {
    ...
}
```

---

# `where`

With many bounds, a signature becomes difficult to read:

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(
    t: &T,
    u: &U,
) -> i32
```

The constraints can be moved out:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    ...
}
```

`where` does not change the meaning.

It only separates:

```text
function structure
from
type constraints
```

and makes complex signatures easier to read.

---

# `impl Trait` as a Return Type

You can hide the concrete return type:

```rust
fn returns_summarizable() -> impl Summary {
    SocialPost {
        ...
    }
}
```

Calling code knows:

```text
the concrete type is unknown

BUT

it implements Summary
```

So `Summary` methods can be used.

---

## Limitation of `-> impl Trait`

An important limitation in the book:

```rust
fn make(switch: bool) -> impl Summary
```

cannot return:

```text
NewsArticle
```

in one branch and:

```text
SocialPost
```

in another, even if both implement `Summary`.

With:

```rust
-> impl Summary
```

the function must return **one concrete type** that implements the trait.

```text
one hidden concrete type
→ ✓

two different concrete types
→ ✗
```

The trait-object solution for different types is postponed until Chapter 17. The Rust Programming Language (2)

---

# Conditional Method Implementations

Traits let you add methods to a generic type only when `T` provides the required behavior.

```rust
struct Pair<T> {
    x: T,
    y: T,
}
```

For any `T`:

```rust
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
```

But comparison and printing can be enabled only with the required traits:

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

So:

```text
Pair<T>::new
→ for any T

cmp_display
→ only if:
  T: Display + PartialOrd
```

This is called a conditional implementation. The Rust Programming Language (2)

---

# Blanket Implementations

A trait can be implemented for **any type** satisfying a particular bound.

The book gives the conceptual standard-library example:

```rust
impl<T: Display> ToString for T {
    ...
}
```

That is:

```text
any T
that implements Display
        ↓
gets ToString
```

Such an implementation is called a **blanket implementation**.

This is why, for example:

```rust
let s = 3.to_string();
```

works through the relationship between implemented traits. The Rust Programming Language (2)

---

# 3. Important Details

## Generics and Traits Solve Different Problems

Do not confuse:

```text
<T>
→ the type is abstracted

T: Trait
→ the abstract type is restricted by behavior
```

For example:

```rust
fn f<T>(x: T)
```

only says:

```text
there is some T
```

while:

```rust
fn f<T: Display>(x: T)
```

also guarantees:

```text
T can be formatted using Display
```

---

## A Trait Bound Determines What Is Allowed Inside a Function

If the signature says:

```rust
T: Summary
```

then the function can rely on `Summary` methods.

If comparison is needed:

```rust
T: PartialOrd
```

If formatting is needed:

```rust
T: Display
```

The compiler checks these requirements at compile time, not runtime. The Rust Programming Language (2)

---

## Generics Do Not Mean Dynamic Typing

For example:

```rust
Vec<T>
```

is not a container for “anything.”

For each concrete instance:

```text
T becomes a concrete type
```

and the compiler knows that type.

Monomorphization turns generic code into specialized code for the concrete types that are used.

---

## A Trait Defines a Behavioral Contract

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

means:

```text
"if a type claims to implement Summary,
it has this method
with this signature"
```

This lets generic code use behavior without knowing the concrete type.

---

## Implementation Restrictions Preserve Coherence

Remember the orphan rule in practical form:

```text
Is the trait ours?
or
Is the type ours?

at least one → implementation is possible

both external → forbidden
```

This prevents conflicting implementations from independent crates.

---

# 4. What to Remember

Minimal generic function:

```rust
fn f<T>(value: T) {
    ...
}
```

Generic structure:

```rust
struct Point<T> {
    x: T,
    y: T,
}
```

Different field types:

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

Implementation:

```rust
impl Summary for MyType {
    fn summarize(&self) -> String {
        ...
    }
}
```

Main model:

```text
TRAIT
→ describes behavior

IMPL TRAIT FOR TYPE
→ says how a concrete type
  provides that behavior
```

---

Trait bound:

```rust
fn f<T: Summary>(x: &T)
```

or:

```rust
fn f(x: &impl Summary)
```

Multiple requirements:

```rust
T: Summary + Display
```

Complex constraints:

```rust
fn f<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
}
```

---

Important difference:

```rust
fn f(a: &impl Summary, b: &impl Summary)
```

→ `a` and `b` may have different types.

```rust
fn f<T: Summary>(a: &T, b: &T)
```

→ `a` and `b` have the same concrete type `T`.

---

Return type:

```rust
fn f() -> impl Summary
```

means:

```text
the concrete type is hidden,
but it implements Summary
```

Within one such function, the returned concrete type must remain the same.

---

Conditional implementation:

```rust
impl<T: Display + PartialOrd> Pair<T> {
    ...
}
```

→ methods exist only for suitable `T`.

Blanket implementation:

```rust
impl<T: Display> ToString for T
```

→ the trait is implemented for all `T` satisfying the bound.

---

Monomorphization:

```text
generic code
    ↓ compile
concrete specialized code
```

So generics provide abstraction without the additional runtime cost described in the book.

---

Final model of the topic:

```text
GENERICS
→ remove type duplication

TRAITS
→ describe shared behavior

TRAIT BOUNDS
→ state what behavior a generic T must provide

MONOMORPHIZATION
→ turns generics into concrete code at compile time
```

If you remember one signature, use this one:

```rust
fn operation<T: Trait>(value: &T)
```

Read it as:

```text
"operation works with any type T,
if that type implements Trait"
```

This is the central connection between generics and traits.

**Notes size:** approximately **10–12 thousand characters** including code.

**Time to copy into a notebook:** approximately **55–70 minutes**. If shortening, the detailed `mixup` example can be removed, but keep monomorphization, `impl Trait`, `<T: Trait>`, `+`, `where`, the orphan rule, conditional implementations, and blanket implementations.
