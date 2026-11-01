# Step 12. Closures and Iterators

## 1. Topic

This chapter combines two Rust features associated with a functional style:

```text
closures
→ anonymous functions
  that can capture their environment

iterators
→ sequential processing of elements
  without manually controlling iteration
```

Closures can be stored in variables and passed to other functions. Their main difference from an ordinary `fn` is that a closure can use values from the scope in which it was created. Iterators, in turn, provide a general way to process elements sequentially. The Rust Programming Language (2)

Main connection:

```text
iterator
   ↓
produces elements
   ↓
closure
   ↓
defines what to do with each element
```

For example:

```rust
values
    .iter()
    .map(|x| x + 1)
    .collect()
```

Here the iterator is responsible for traversal, while the closure:

```rust
|x| x + 1
```

defines the concrete operation.

---

# 2. Mechanism

## Closure

The simplest closure:

```rust
let add_one = |x| x + 1;
```

Call:

```rust
let result = add_one(5);
```

Parameters are written between:

```text
| |
```

For example:

```rust
|x| x + 1
```

Several parameters:

```rust
|x, y| x + y
```

No parameters:

```rust
|| something()
```

---

## Closure and Ordinary Function

The same behavior can be written as:

```rust
fn add_one(x: u32) -> u32 {
    x + 1
}
```

or:

```rust
let add_one = |x: u32| -> u32 {
    x + 1
};
```

But closure types are usually inferred:

```rust
let add_one = |x| x + 1;
```

If the body contains one expression, the braces can also be omitted. The Rust Programming Language (2)

---

## A Closure's Types Become Fixed

The absence of explicit annotations does not make one closure generic.

For example:

```rust
let identity = |x| x;

let s = identity(String::from("hello"));
let n = identity(5);
```

does not compile.

After the first call:

```text
identity(String)
        ↓
parameter = String
return    = String
```

these concrete types are fixed for that closure.

```text
closure without annotations
≠ generic function
```

The Rust Programming Language (2)

---

# Capturing the Environment

The main difference between a closure and an ordinary function:

```rust
let x = 10;

let f = || println!("{x}");
```

`x` is not a parameter of `f`, but the closure can use it.

A closure automatically chooses the least powerful capture mode it needs:

```text
&T
→ immutable borrow

&mut T
→ mutable borrow

T
→ take ownership
```

These modes correspond to the ownership and borrowing rules already studied. The Rust Programming Language (2)

---

## Immutable Capture

```rust
let list = vec![1, 2, 3];

let only_borrows =
    || println!("{list:?}");

only_borrows();
```

The closure only needs to read `list`, so it captures an immutable reference.

Conceptually:

```text
closure
   │
   └── &list
```

`list` itself remains available wherever ordinary immutable borrowing rules allow it.

---

## Mutable Capture

```rust
let mut list = vec![1, 2, 3];

let mut add =
    || list.push(7);

add();
```

Now the closure modifies `list`, so it captures:

```text
&mut list
```

While this mutable borrow is active, the ordinary `&mut` rules apply: conflicting access to `list` is not allowed. The Rust Programming Language (2)

---

# `move`

Captured values can be forced into the closure by ownership:

```rust
move || {
    ...
}
```

For example:

```rust
let list = vec![1, 2, 3];

let closure =
    move || println!("{list:?}");
```

Meaning:

```text
before:

scope owns list


move closure
     ↓


closure owns list
```

The book especially connects this with sending a closure to another thread:

```rust
thread::spawn(
    move || println!("{list:?}")
);
```

Now the data belongs to the closure itself and does not depend on whether the scope where the closure was created has already ended. The Rust Programming Language (2)

Important:

```text
move
→ determines how a value enters the closure
```

Which `Fn` trait the closure implements also depends on **what the closure body later does with the captured value**.

---

# `Fn`, `FnMut`, `FnOnce`

Rust automatically determines one or more special traits for a closure.

## `FnOnce`

Every closure implements at least:

```rust
FnOnce
```

If the body moves a captured value **out of the closure itself**, the closure can be called only once.

```text
closure owns value
      ↓
call
      ↓
value moved out
      ↓
second call impossible
```

Such a closure may implement only `FnOnce`.

---

## `FnMut`

```rust
FnMut
```

fits closures that:

```text
do not move captured data out
+
may modify their environment
```

For example:

```rust
let mut count = 0;

let mut f = || {
    count += 1;
};
```

The closure can be called several times, but it modifies captured state.

---

## `Fn`

```rust
Fn
```

fits closures that:

```text
do not move captured values out
do not modify them
```

or capture nothing at all.

For example:

```rust
|x| x + 1
```

Such a closure can be called repeatedly without changing its environment. The Rust Programming Language (2)

Main model:

```text
FnOnce
→ can be called at least once

FnMut
→ can be called repeatedly
→ may modify the environment

Fn
→ can be called repeatedly
→ does not modify the environment
```

The implementation relationship is additive:

```text
Fn
→ also satisfies FnMut
→ also satisfies FnOnce
```

---

## Closure Traits as Bounds

A function can accept a closure through a generic parameter plus a trait bound.

Simplified example from `Option::unwrap_or_else`:

```rust
F: FnOnce() -> T
```

This means:

```text
F
→ closure

()
→ no arguments

-> T
→ returns T

FnOnce
→ will be called at most once
```

`unwrap_or_else` calls the closure only when the `Option` contains `None`, so `FnOnce` is sufficient. The Rust Programming Language (2)

Another example:

```text
sort_by_key
→ calls the closure many times
→ requires FnMut
```

This leads to an important principle:

> `Fn` / `FnMut` / `FnOnce` in a signature describe how the calling code intends to use the supplied closure.

---

# Iterators

An iterator lets you obtain elements sequentially without manually implementing:

```text
an index
end checking
counter increment
access to the next element
```

Simplest example:

```rust
let v = vec![1, 2, 3];

let iter = v.iter();
```

Creating:

```rust
v.iter()
```

does not yet process the elements.

Rust iterators are **lazy**. The Rust Programming Language (2)

---

# The `Iterator` Trait

Simplified definition:

```rust
pub trait Iterator {
    type Item;

    fn next(
        &mut self
    ) -> Option<Self::Item>;
}
```

The key method to implement for your own iterator is:

```rust
next()
```

Each call returns:

```text
another element exists
→ Some(item)

no elements remain
→ None
```

The Rust Programming Language (2)

---

## `next()`

```rust
let v = vec![1, 2, 3];

let mut iter = v.iter();

assert_eq!(
    iter.next(),
    Some(&1)
);

assert_eq!(
    iter.next(),
    Some(&2)
);

assert_eq!(
    iter.next(),
    Some(&3)
);

assert_eq!(
    iter.next(),
    None
);
```

The iterator must be `mut` because every `next()` changes its internal state:

```text
position 0
   ↓ next
position 1
   ↓ next
position 2
...
```

The Rust Programming Language (2)

---

# `iter`, `iter_mut`, `into_iter`

How an iterator is created determines its relationship to the original values.

```rust
collection.iter()
```

iterates over:

```text
&T
```

So the values are only borrowed.

---

```rust
collection.iter_mut()
```

produces:

```text
&mut T
```

and allows the elements to be modified.

---

```rust
collection.into_iter()
```

creates an iterator that takes ownership of the collection and yields owned values.

Model:

```text
iter()
→ &T
→ read

iter_mut()
→ &mut T
→ modify

into_iter()
→ T
→ take ownership of elements
```

The Rust Programming Language (2)

This is one of the most important practical distinctions when working with iterators.

---

# Consuming Methods

Some methods run through an iterator to the end and **consume it**.

Example:

```rust
let v = vec![1, 2, 3];

let total: i32 =
    v.iter().sum();
```

Result:

```text
6
```

`sum` repeatedly calls `next` until the sequence is exhausted.

After consumption, that iterator cannot be used again because `sum` takes ownership of it. The Rust Programming Language (2)

Main model:

```text
iterator
   ↓
consumer
   ↓
final result
```

Examples from the chapter:

```text
sum()
collect()
```

---

# Iterator Adapters

Other methods do not calculate a final result; they create a **new iterator**.

For example:

```rust
map
filter
```

These are called iterator adapters.

```text
iterator
    ↓
adapter
    ↓
new iterator
```

---

## `map`

`map` transforms every element:

```rust
let v = vec![1, 2, 3];

let result: Vec<_> =
    v.iter()
        .map(|x| x + 1)
        .collect();
```

Result:

```text
[2, 3, 4]
```

Logic:

```text
1 → closure → 2
2 → closure → 3
3 → closure → 4
```

But:

```rust
v.iter().map(|x| x + 1);
```

by itself effectively does no work.

Why?

```text
map
→ creates a new iterator

iterator
→ lazy
```

To actually compute the values, the iterator must be consumed, for example with `collect`. The Rust Programming Language (2)

---

## `filter`

`filter` keeps only the elements for which the closure returns:

```text
true
```

For example:

```rust
shoes
    .into_iter()
    .filter(|s| {
        s.size == shoe_size
    })
    .collect()
```

Logic:

```text
element
   ↓
closure
   ↓
 bool
 ┌─┴────┐
true   false
 ↓       ↓
keep    skip
```

The closure can use `shoe_size` from the environment — this is exactly the connection between iterators and closures. The Rust Programming Language (2)

---

# `collect`

`collect()` consumes an iterator and gathers its elements into a collection.

For example:

```rust
let result: Vec<_> =
    v.iter()
        .map(|x| x + 1)
        .collect();
```

Why is the result type often required?

Because `collect` can gather elements into different kinds of collections, and Rust needs to know which one is requested.

In the example:

```rust
Vec<_>
```

means:

```text
collect into a Vec
```

---

# Iterator Chains

The power of iterators appears in composition:

```rust
collection
    .into_iter()
    .filter(|x| ...)
    .map(|x| ...)
    .collect()
```

Read it from top to bottom:

```text
obtain elements
       ↓
filter them
       ↓
transform them
       ↓
collect the result
```

Intermediate adapters:

```text
filter
map
...
```

are lazy.

Execution begins when there is a consuming operation:

```text
collect
sum
for
...
```

---

# 3. Important Details

## Iterator Laziness

This is a central property.

```rust
let iter =
    v.iter()
        .map(|x| x + 1);
```

does not mean:

```text
"walk through all of v now
and create every result"
```

Until the iterator is consumed, the work is not performed.

```text
creating the pipeline
→ describes a computation

consuming operation
→ starts the computation
```

For this reason, Rust even warns when `map(...)` is created and then never used. The Rust Programming Language (2)

---

## Closures Still Follow Ownership

A closure is not a way around the borrow checker.

If it captures:

```text
&T
```

the immutable-borrow rules apply.

If it captures:

```text
&mut T
```

the mutable-borrow rules apply.

If it captures:

```text
T
```

the move rules apply.

So errors around closures are often familiar ownership errors, except the value is now stored inside the closure.

---

## `move` and `FnOnce` Are Not the Same Thing

Do not confuse:

```text
move
→ forces the closure to take ownership
  of captured values
```

with:

```text
FnOnce
→ describes whether the body moves a value
  OUT OF the closure when called
```

The mere presence of `move` does not automatically mean the closure can be called only once. Classification depends on what the body does with the captured values. This follows from the book's separation between capture mode and the later behavior of the closure body. The Rust Programming Language (2)

---

## Iterators and Ownership

It is very useful to inspect the beginning of a pipeline:

```rust
v.iter()
```

→ elements are borrowed.

```rust
v.iter_mut()
```

→ elements are mutably borrowed.

```rust
v.into_iter()
```

→ ownership is transferred to the iterator.

So even before reading `map/filter`, you can understand what will happen to the original collection.

---

## Iterators Instead of Manual Temporary State

Old search implementation:

```rust
let mut results = Vec::new();

for line in contents.lines() {
    if line.contains(query) {
        results.push(line);
    }
}

results
```

After switching to iterators:

```rust
contents
    .lines()
    .filter(|line| {
        line.contains(query)
    })
    .collect()
```

The meaning is the same:

```text
lines
→ keep matching lines
→ collect
```

but these disappear:

```text
mutable results
manual push
explicit loop control
```

The book notes that Rust developers usually prefer iterator style after learning the adapters because it focuses on the goal of the transformation rather than the mechanics of iteration. The Rust Programming Language (2)

---

## Practical Improvement to `Config::build`

Previously CLI arguments were first collected:

```rust
let args: Vec<String> =
    env::args().collect();
```

and values were then cloned into `Config`.

After learning iterators, the book passes `env::args()` directly:

```rust
Config::build(env::args())
```

and `build` accepts:

```rust
impl Iterator<Item = String>
```

This lets it obtain arguments through `next()` and **move the `String`s directly into `Config`**, instead of borrowing slice elements and then cloning them. The Rust Programming Language (2)

This is a good example showing that an iterator is not just prettier syntax:

```text
owning iterator
→ may eliminate an unnecessary clone
```

---

## Performance

The book compares search implemented with an ordinary loop and with an iterator pipeline and shows similar performance. Iterators are described as one of Rust's **zero-cost abstractions**: high-level code compiles to roughly the same efficient low-level code one could write manually. The Rust Programming Language (2)

Main point:

```text
iterator abstraction
≠ mandatory runtime cost
```

The compiler can optimize iterator chains, including removing some loop bookkeeping. The Rust Programming Language (2)

---

# 4. What to Remember

Minimum for closures:

```rust
let f = |x| x + 1;
```

```text
|x|
→ parameters

x + 1
→ body
```

A closure:

```text
→ can be stored in a variable
→ can be passed to a function
→ can capture its environment
```

---

Three capture modes:

```text
&T
→ read

&mut T
→ modify

T
→ ownership
```

Forced ownership:

```rust
move || ...
```

---

Three `Fn` traits:

```text
FnOnce
→ may consume a captured value
→ at most one suitable call is guaranteed

FnMut
→ does not consume the captured value
→ may modify the environment
→ repeated calls

Fn
→ does not consume
→ does not modify the environment
→ repeated calls
```

Practically:

```text
Fn
⊂ FnMut
⊂ FnOnce
```

in the sense that a closure with more restrictive behavior also satisfies the more general calling requirements.

---

Minimum for iterators:

```rust
let iter = v.iter();
```

Main trait:

```rust
trait Iterator {
    type Item;

    fn next(
        &mut self
    ) -> Option<Self::Item>;
}
```

`next`:

```text
Some(item)
Some(item)
...
None
```

---

Ownership choice:

```text
iter()
→ &T

iter_mut()
→ &mut T

into_iter()
→ T
```

---

Main methods:

```text
map
→ transform each element
→ returns an iterator

filter
→ keep matching elements
→ returns an iterator

collect
→ gather an iterator into a collection
→ consumes the iterator

sum
→ obtain a sum
→ consumes the iterator
```

---

Typical pipeline:

```rust
items
    .into_iter()
    .filter(|x| condition(x))
    .map(|x| transform(x))
    .collect()
```

Read it as:

```text
obtain elements
→ filter
→ transform
→ collect
```

---

Most important property:

```text
iterator adapters
→ lazy
```

Therefore:

```rust
v.iter().map(|x| x + 1);
```

by itself does not perform all the work.

A consumer is required:

```rust
.collect()
```

or another way to consume the iterator.

---

Final model of the topic:

```text
CLOSURE
→ describes an action

ITERATOR
→ provides a sequence

ADAPTER
→ transforms the sequence

CONSUMER
→ starts processing
  and obtains the result
```

Connection with previous chapters:

```text
closures
→ ownership + borrowing + traits

iterators
→ traits + Option + closures

iterator pipelines
→ compact processing description
  without manual mutable state
```

Main practical rule when reading iterator code:

```text
1. Where do elements come from?
   iter / iter_mut / into_iter

2. What do the adapters do?
   map / filter / ...

3. Where is the iterator consumed?
   collect / sum / for / ...
```

If you answer these three questions, most iterator chains become much easier to read.

**Notes size:** approximately **10–12 thousand characters** including code and diagrams.

**Time to copy into a notebook:** approximately **55–70 minutes**. If shortening, examples involving `sort_by_key` and `Config::build` can be removed, but keep environment capture, `move`, `Fn/FnMut/FnOnce`, `Iterator::next`, `iter/iter_mut/into_iter`, laziness, and the `map/filter/collect` combination.
