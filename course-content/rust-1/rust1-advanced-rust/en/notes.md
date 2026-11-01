# Step 15. Complete Language Model: Trait Objects, Patterns, Unsafe, Macros

## 1. Topic

The final step combines features that are needed less often than basic ownership-oriented code but complete the overall Rust model:

```text
trait objects
→ different concrete types
  with shared behavior

patterns
→ decompose the structure of a value

unsafe
→ operations whose safety
  the compiler cannot prove

macros
→ generate Rust code
  at compile time
```

They are accompanied by advanced traits and types: associated types, fully qualified syntax, newtype, `Sized`, and function pointers. Chapter 19 itself presents these capabilities as tools for specific situations and as reference material rather than everyday basic syntax. The Rust Programming Language(2)

The overall model after the entire book:

```text
ownership / borrowing
→ who owns data and who can access it

types / generics / traits
→ which values are allowed and which behavior is guaranteed

patterns
→ how to decompose a received value

trait objects
→ how to select a behavior implementation at runtime

unsafe
→ where the programmer manually guarantees invariants

macros
→ how to generate code at compile time
```

---

## 2. Mechanism

### Trait objects: `dyn Trait`

Generics with trait bounds work when the compiler knows the concrete type:

```rust
fn draw<T: Draw>(item: &T) {
    item.draw();
}
```

For every actually used `T`, monomorphization occurs:

```text
T = Button
→ separate concrete code

T = SelectBox
→ separate concrete code
```

This is **static dispatch**: the concrete method to call is known at compile time.

Sometimes, however, one collection must contain different types:

```text
Button
SelectBox
TextField
...
```

if all of them implement:

```rust
trait Draw {
    fn draw(&self);
}
```

Then a trait object is used:

```rust
Box<dyn Draw>
```

For example:

```rust
struct Screen {
    components: Vec<Box<dyn Draw>>,
}
```

Now a single `Vec` can contain different concrete types as long as each implements `Draw`. The Rust Programming Language(2)

```text
Vec<Box<dyn Draw>>
        │
        ├── Box<Button>
        ├── Box<SelectBox>
        └── Box<...>
```

`Screen` itself only knows:

```text
draw() can be called on every element
```

It does not need to know the concrete type.

---

### Why a pointer is required

A trait object is used behind some kind of pointer:

```rust
&dyn Draw
Box<dyn Draw>
```

The concrete size of the value hidden behind `dyn Draw` is not known in advance, so `dyn Draw` cannot be stored directly as an ordinary sized value.

A trait object connects data of a concrete type with the information needed to find that trait's method implementations at runtime. The Rust Programming Language(2)

---

### Dynamic dispatch

With:

```rust
component.draw();
```

where:

```rust
component: Box<dyn Draw>
```

the compiler does not know in advance whether the value inside is a `Button` or some other type.

Therefore implementation selection happens at runtime:

```text
trait object
     ↓ runtime
identify the concrete type / required method
     ↓
call the corresponding draw
```

This is **dynamic dispatch**.

Unlike generics:

```text
<T: Draw>
→ static dispatch
→ monomorphization

dyn Draw
→ dynamic dispatch
→ method selection at runtime
```

Dynamic dispatch has runtime cost and limits some optimizations such as inlining. The Rust Programming Language(2)

---

### When to use generics and when to use trait objects

If a collection is homogeneous:

```rust
struct Screen<T: Draw> {
    components: Vec<T>,
}
```

all components in a particular `Screen<T>` have one concrete `T`.

If heterogeneity is required:

```rust
Vec<Box<dyn Draw>>
```

can store different types.

In summary:

```text
generic T
→ one concrete type for a particular instance
→ static dispatch

dyn Trait
→ different concrete types
→ shared behavior
→ dynamic dispatch
```

---

### State pattern

The book demonstrates trait objects with the state pattern.

There are states:

```text
Draft
→ PendingReview
→ Published
```

Each state implements:

```rust
trait State
```

And `Post` stores:

```rust
state: Option<Box<dyn State>>
```

A state transition is performed through methods on `State` itself, which consume the old state and return a new one. The Rust Programming Language(2)

Conceptually:

```text
Post
 │
 └── Box<dyn State>
         │
         ├─ Draft
         ├─ PendingReview
         └─ Published
```

`Post` delegates behavior to the current state object.

Advantage:

```text
logic for a particular state
is concentrated in one type
```

The drawback is that states become coupled through transitions, and some logic may be duplicated. The Rust Programming Language(2)

---

### State can be encoded in types

The book shows an alternative that makes stronger use of Rust's type system:

```text
DraftPost
→ PendingReviewPost
→ Post
```

`DraftPost` can simply have no method named:

```rust
content()
```

Then trying to read an unpublished post becomes a compile-time error rather than a runtime check.

```text
invalid state
→ cannot be expressed as correctly typed code
```

This is an important general Rust idea:

> if a state must be forbidden, it can sometimes be better to make that state impossible through the type system rather than check it with `if` at runtime. The Rust Programming Language(2)

---

## Patterns

A pattern describes the **shape of a value**.

We have already used them:

```rust
match value {
    Some(x) => ...
    None => ...
}
```

The pattern:

```rust
Some(x)
```

simultaneously:

```text
checks:
is value the Some variant?

and

extracts:
inner value → x
```

But patterns are used in more places than `match`.

---

### Where patterns appear

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

The loop continues while the pattern matches. The Rust Programming Language(2)

#### `for`

Even:

```rust
for x in values
```

contains the pattern `x`.

You can destructure immediately:

```rust
for (index, value) in items.iter().enumerate() {
    ...
}
```

#### `let`

```rust
let (x, y) = (1, 2);
```

#### Function parameters

```rust
fn print(&(x, y): &(i32, i32)) {
    ...
}
```

Patterns are built more deeply into the language than just `match`.

---

### Irrefutable and refutable patterns

An **irrefutable pattern** always matches.

```rust
let x = 5;
let (x, y) = (1, 2);
```

Such patterns are suitable for `let` and function parameters.

A **refutable pattern** may fail to match:

```rust
Some(x)
```

because the value might be `None`.

Therefore:

```rust
let Some(x) = value;
```

is not generally suitable as an ordinary unconditional binding.

For such cases, use:

```text
match
if let
while let
```

The book emphasizes the distinction between refutable and irrefutable patterns as the basis for understanding where a particular pattern is allowed. The Rust Programming Language(2)

---

### Destructuring

Structures:

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

Patterns can be nested:

```rust
match value {
    Some(Point { x, y }) => ...
    None => ...
}
```

So a pattern can mirror the structure of data while extracting the required parts.

---

### Multiple alternatives: `|`

```rust
match x {
    1 | 2 => println!("one or two"),
    _ => {}
}
```

Means:

```text
pattern A
OR
pattern B
```

---

### Ranges

Numbers and `char` values can be matched against a range:

```rust
match x {
    1..=5 => ...
    _ => ...
}
```

---

### Ignoring with `_`

```rust
match value {
    Some(_) => println!("there is a value"),
    None => {}
}
```

`_`:

```text
matches
+
does not bind the value
```

This differs from:

```rust
_unused
```

A name beginning with `_` **is still a binding** and can take ownership of the value; it merely suppresses the unused-variable warning.

---

### `..`

Allows the remaining parts of a structure to be ignored:

```rust
let Point { x, .. } = point;
```

or:

```rust
let (first, .., last) = tuple;
```

But `..` must be unambiguous: the compiler must be able to determine exactly which values are being skipped. The Rust Programming Language(2)

---

### Match guards

An additional condition can follow a pattern:

```rust
match num {
    Some(x) if x % 2 == 0 => ...
    Some(x) => ...
    None => ...
}
```

The pattern must match first, then this is checked:

```rust
if condition
```

This is called a **match guard**. The Rust Programming Language(2)

Important: a guard is additional logic and does not participate in normal pattern exhaustiveness checking in the same way as the patterns themselves.

---

### `@` binding

Sometimes you need to do both:

```text
check the shape of a value
+
keep the entire matched value
```

For this:

```rust
id @ 3..=7
```

Conceptually:

```text
3..=7
→ checks the range

id
→ receives the matched value
```

`@` combines a pattern check with a binding.

---

## Unsafe Rust

Safe Rust is intentionally conservative: if the compiler cannot prove that an operation is safe, it may reject even code whose correctness the programmer can prove independently.

For such cases there is:

```rust
unsafe {
    ...
}
```

But `unsafe` **does not disable the entire borrow checker or type system**.

It only allows a specific set of operations whose safety the programmer is now responsible for guaranteeing manually. The Rust Programming Language(2)

---

### Five capabilities of `unsafe`

The book lists five operations:

```text
1. Dereference a raw pointer.

2. Call an unsafe function / method.

3. Read or modify a mutable static.

4. Implement an unsafe trait.

5. Access fields of a union.
```

The Rust Programming Language(2)

`unsafe` itself does not mean the code is necessarily incorrect. It means:

```text
compiler cannot verify this part
→ programmer guarantees invariants
```

---

### Raw pointers

There are two forms:

```rust
*const T
*mut T
```

Unlike ordinary references, raw pointers:

```text
may ignore ordinary reference guarantees
may be null
may point to invalid memory
do not receive automatic borrowing guarantees
```

A raw pointer can be created without `unsafe`, but it can only be **dereferenced** inside `unsafe`.

```rust
unsafe {
    println!("{}", *ptr);
}
```

The memory access operation itself is what is dangerous.

---

### Unsafe function

A function can declare:

```rust
unsafe fn dangerous() {
    ...
}
```

Calling it requires:

```rust
unsafe {
    dangerous();
}
```

The meaning of `unsafe fn` is:

```text
the function has additional conditions
that the compiler cannot verify
```

Calling code is responsible for satisfying those conditions.

---

### Safe abstraction around unsafe

One of the most important principles of the chapter:

```text
unsafe implementation
can have a
safe public API
```

The book demonstrates this using an analogue of `split_at_mut`.

The borrow checker cannot independently prove that the two returned `&mut` references point to non-overlapping parts of a single slice.

The implementation uses raw pointers and `unsafe`, but first checks the `mid` boundary and exposes only ordinary safe references to callers.

```text
public safe function
        ↓
checks invariants
        ↓
small unsafe block
        ↓
returns safe abstraction
```

This approach limits the region in which the programmer must manually guarantee safety. The Rust Programming Language(2)

---

### FFI and `extern`

Rust can interact with functions from other languages through FFI.

For example, declaring a function with the C ABI uses:

```rust
extern "C" {
    ...
}
```

`"C"` specifies the ABI — the convention for how the function is called at the binary level.

Calling an external function is unsafe because Rust cannot verify the guarantees of code written outside Rust. The Rust Programming Language(2)

---

## Advanced traits and types

This is secondary material in this step, but several constructs should be recognizable.

### Associated types

We have already seen:

```rust
trait Iterator {
    type Item;

    fn next(&mut self)
        -> Option<Self::Item>;
}
```

`Item` is an associated type.

An `Iterator` implementation chooses a concrete `Item` once:

```text
Iterator for Counter
→ Item = u32
```

Unlike a generic trait parameter, an associated type becomes part of the specific implementation's contract. The Rust Programming Language(2)

---

### Operator overloading

Rust associates operators with traits.

For example:

```text
+
→ Add
```

You can implement:

```rust
impl Add for Point {
    ...
}
```

and define the meaning of:

```rust
point1 + point2
```

Arbitrary new operators cannot be created; supported operations are overloaded through their corresponding traits. The Rust Programming Language(2)

---

### Fully qualified syntax

If several traits and the type itself have methods with the same name, the desired implementation can be selected explicitly:

```rust
Pilot::fly(&person);
Wizard::fly(&person);
Human::fly(&person);
```

For especially ambiguous associated functions, use the full syntax:

```rust
<Type as Trait>::function(...)
```

It tells the compiler exactly:

```text
which Trait
for which Type
which function
must be called
```

The Rust Programming Language(2)

---

### Newtype

The orphan rule prevents directly implementing a foreign trait for a foreign type.

A solution is:

```rust
struct Wrapper(Vec<String>);
```

`Wrapper` is a new local type.

Now it can have:

```rust
impl Display for Wrapper {
    ...
}
```

Newtype:

```text
creates a genuinely new type
→ provides type safety
→ allows trait implementations
```

The book notes that such a wrapper is removed during compilation and does not itself add runtime cost. The Rust Programming Language(2)

Do not confuse it with a type alias:

```rust
type Kilometers = i32;
```

An alias is only another name for the same type:

```text
Kilometers == i32
```

It does not create additional type safety. The Rust Programming Language(2)

---

### DST and `Sized`

Some types do not have a known compile-time size:

```text
str
[T]
dyn Trait
```

Such values are used behind pointers:

```rust
&str
&[T]
Box<dyn Trait>
```

A generic parameter implicitly has this bound by default:

```rust
T: Sized
```

To permit a type whose size is not known:

```rust
T: ?Sized
```

But then `T` is normally handled through a reference or another pointer. The Rust Programming Language(2)

---

### Function pointers

An ordinary function has the type:

```rust
fn
```

For example:

```rust
fn do_twice(
    f: fn(i32) -> i32,
    value: i32,
) -> i32 {
    f(value) + f(value)
}
```

This is specifically a **function pointer type**, not the `Fn` trait.

Function pointers implement `Fn`, `FnMut`, and `FnOnce`, so they can be passed where a suitable closure is expected. The Rust Programming Language(2)

---

## Macros

A macro is a **metaprogramming** mechanism:

```text
Rust code
→ macro
→ generates other Rust code
→ that code is then compiled
```

We have already used:

```rust
println!()
vec![]
panic!()
```

Macros differ from functions because they operate on code at compile time. They can also accept a variable number of arguments and generate constructs that a runtime function cannot create, such as a trait implementation. The Rust Programming Language(2)

The cost is that macro definitions are more difficult to read and maintain.

---

### Declarative macros: `macro_rules!`

Basic form:

```rust
macro_rules! name {
    (pattern) => {
        generated_code
    };
}
```

Conceptually, this resembles `match`, except that it matches the structure of source Rust code rather than a runtime value.

```text
input tokens
     ↓
pattern matching
     ↓
generated Rust code
```

The Rust Programming Language(2)

A simplified idea of `vec!`:

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

Main elements:

```text
$x:expr
→ capture a Rust expression

$( ... ),*
→ repeat separated by commas
  zero or more times

$( ... )*
→ repeat the generated code
```

Therefore:

```rust
vec![1, 2, 3]
```

generates several `push` calls. The Rust Programming Language(2)

---

### Procedural macros

The second major category accepts source code as a:

```rust
TokenStream
```

and returns a new:

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

The Rust Programming Language(2)

There are three kinds:

```text
custom derive
attribute-like
function-like
```

---

### Custom derive

Example use:

```rust
#[derive(HelloMacro)]
struct Pancakes;
```

The macro generates trait implementation code for the type.

Conceptually:

```text
struct Pancakes
        ↓ derive macro
generated:
impl HelloMacro for Pancakes { ... }
```

For working with code, the book shows `proc_macro`, `syn`, and `quote`: `syn` parses Rust code into a data structure, while `quote!` helps generate new Rust code. The Rust Programming Language(2)

---

### Attribute-like

Example form:

```rust
#[route(GET, "/")]
fn index() {
    ...
}
```

Such a macro receives separately:

```text
attribute contents
+
annotated item
```

and generates new code. The Rust Programming Language(2)

---

### Function-like procedural macro

It looks like an ordinary macro call:

```rust
sql!(SELECT ...);
```

Internally, however, it can perform complex parsing of the input `TokenStream` and then return generated Rust code. The Rust Programming Language(2)

---

## 3. Important details

### Static and dynamic dispatch

This is one of the key distinctions in the later chapters:

```text
GENERICS

T: Trait
→ concrete type is known at compile time
→ monomorphization
→ static dispatch
```

versus:

```text
TRAIT OBJECT

dyn Trait
→ concrete type is hidden
→ runtime method selection
→ dynamic dispatch
```

Trait objects are not a replacement for generics in general; they are needed where runtime polymorphism is genuinely part of the model.

---

### Patterns are not just `switch`

A pattern can:

```text
check a variant
+
check a literal/range
+
destructure a struct
+
extract fields
+
ignore parts
+
create bindings
```

Therefore:

```rust
Some(Point { x, .. })
```

expresses considerably more than an ordinary value comparison.

---

### Shadowing inside a pattern

A named pattern creates a new variable.

For example, an outer variable:

```rust
let y = 10;
```

and:

```rust
match x {
    Some(y) => ...
    _ => ...
}
```

The inner `y` is a new binding that shadows the outer `y`.

If you need to compare against the outer value, a match guard may be the appropriate mechanism.

---

### `_` and `_name` are different

```rust
_
```

does not bind the value.

```rust
_value
```

binds it, merely suppressing the unused-variable warning.

This can matter for ownership: `_value` can move a value, while `_` does not.

---

### `unsafe` is best localized

An ideal structure:

```text
large safe codebase

        ↓

small safe abstraction
        ↓
tiny unsafe implementation
```

Then only a minimal region needs to be verified manually.

`unsafe` should not be used merely to bypass an inconvenient compiler error without proving the required invariants.

---

### Memory safety does not mean absence of all errors

Even safe Rust does not eliminate:

```text
business-logic errors
deadlock
memory leak through an Rc cycle
incorrect algorithms
```

`unsafe` primarily concerns guarantees that the compiler cannot enforce for certain low-level operations.

---

### Trait objects are related to DSTs

```rust
dyn Trait
```

does not have one predetermined size because the concrete type can vary.

Therefore it is used as:

```rust
&dyn Trait
Box<dyn Trait>
```

This connects Chapter 17 with `Sized/?Sized` from Chapter 19.

---

### Macros should be distinguished from functions

```text
FUNCTION
→ receives runtime values
→ executes at runtime

MACRO
→ receives syntax/tokens
→ generates Rust code
→ works before normal execution
```

If a task can be solved with an ordinary function, a function is usually simpler. A macro is needed when you must operate on code structure, vary the number of arguments, or generate new constructs.

---

### How Chapter 20 connects the whole book

The final web-server project does not introduce a fundamentally new language model. It combines material learned earlier:

```text
TcpListener / IO
→ working with an external system

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

For example, the `thread::spawn` signature again connects a closure, `FnOnce`, `Send`, and `'static`, while the thread pool stores `JoinHandle<()>` values and sends work to threads.

So Chapter 20 demonstrates not a separate “web framework” but an assembly of mechanisms studied earlier into one application.

---

## 4. What to remember

### Trait objects

```rust
Box<dyn Trait>
&dyn Trait
```

mean:

```text
concrete type is unknown
but
Trait is implemented
```

The main distinction:

```text
<T: Trait>
→ static dispatch
→ one concrete type
→ monomorphization

dyn Trait
→ dynamic dispatch
→ different concrete types
→ runtime choice
```

Typical case:

```rust
Vec<Box<dyn Draw>>
```

→ a heterogeneous collection with shared behavior.

---

### Patterns

Where they are used:

```text
match
if let
while let
for
let
function parameters
```

Main constructs:

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

Remember:

```text
pattern
→ not only checks a value
→ but also destructures it
→ and creates bindings
```

And the distinction:

```text
irrefutable
→ always matches
→ let / function params

refutable
→ may fail to match
→ match / if let / while let
```

---

### Unsafe

```rust
unsafe {
    ...
}
```

does not disable Rust as a whole.

It permits five special operations:

```text
raw pointer dereference
unsafe function/method call
mutable static access
unsafe trait implementation
union field access
```

The main model:

```text
unsafe
→ compiler no longer proves
  a particular invariant

→ responsibility moves to the programmer
```

The best architectural pattern from the chapter:

```text
safe API
   ↓
small unsafe implementation
```

---

### Advanced constructs

```text
Iterator::Item
→ associated type

<Type as Trait>::method
→ explicitly select an implementation

struct Wrapper(T)
→ newtype

type Alias = ExistingType
→ only an alias, not a new type

T: ?Sized
→ T may have an unknown compile-time size

fn(i32) -> i32
→ function pointer
```

---

### Macros

Two main families:

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

The key difference from a function:

```text
function
→ works with values

macro
→ works with code
→ generates code at compile time
```

---

### Final Rust model

After all 15 steps, the language can be reduced to one chain:

```text
VALUES
↓
have TYPES

TYPES
↓
have OWNERSHIP

OWNERSHIP
↓
controls data lifetime

REFERENCES
↓
provide borrowing without transferring ownership

LIFETIMES
↓
prove reference validity

GENERICS
↓
abstract over concrete types

TRAITS
↓
define required behavior

TRAIT OBJECTS
↓
provide runtime polymorphism

PATTERNS
↓
decompose the structure of values

RESULT / OPTION
↓
explicitly represent alternative states

SMART POINTERS
↓
extend ownership models

SEND / SYNC + THREADS
↓
carry the ownership model
into concurrency

UNSAFE
↓
locally transfers part of the safety proof
to the programmer

MACROS
↓
generate Rust code at compile time
```

The most important idea of the entire book is that **Rust tries to represent as many program properties as possible through types, ownership, traits, and compile-time checks. Runtime mechanisms such as trait objects, `RefCell`, `Mutex`, or dynamic dispatch are used where the static model is insufficient or additional flexibility is required; `unsafe` goes one level deeper, where the programmer must prove the required property manually.**

**Note length:** approximately **14–16 thousand characters** including code and diagrams.

**Time to copy into a notebook:** approximately **80–95 minutes**. If you need to stay closer to one hour, you can shorten the state pattern, associated types/newtype/function pointers, and details of procedural macros; the core of the final step is `dyn Trait`, static vs dynamic dispatch, refutable/irrefutable patterns, destructuring, the five capabilities of `unsafe`, safe abstraction around unsafe, and the difference between `macro_rules!` and procedural macros.
