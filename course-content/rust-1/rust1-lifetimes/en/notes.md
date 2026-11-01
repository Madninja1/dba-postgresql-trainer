# Step 10. Lifetimes

## 1. Topic

A **lifetime is the lifetime of a reference** — the region of a program during which that reference remains valid.

Every reference in Rust has a lifetime even if it is never written explicitly. In most situations, the compiler infers it automatically. An explicit annotation is required when Rust cannot determine the relationship between several references unambiguously. The Rust Programming Language (2)

The main purpose of lifetimes is:

```text
not to make data live longer

but to prove to the compiler that:

a reference will never outlive
the data it points to
```

In other words:

```text
data lifetime
    ≥
reference lifetime
```

If the data is destroyed before the reference:

```text
dangling reference
→ compilation is rejected
```

---

# 2. Mechanism

## Why Lifetimes Are Needed

Consider:

```rust
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("{r}");
}
```

In terms of scopes:

```text
r  ──────────────────────────────┐
                                 │
       x ───────────┐             │
                    │             │
                    ↓             │
                 x destroyed      │
                                  │
                       r is used
```

`r` lives longer than `x`.

After the inner block ends:

```text
x
→ destroyed

r
→ still exists
→ but would point to a destroyed value
```

Rust rejects this code.

The borrow checker compares the lifetimes of data and references and verifies that each reference remains valid for as long as it is used. The Rust Programming Language (2)

---

## Safe Version

```rust
fn main() {
    let x = 5;

    let r = &x;

    println!("{r}");
}
```

Now:

```text
x  ────────────────────────┐
                           │
     r ───────────────┐     │
                      │     │
                  use r     │
```

The data `x` exists longer than the reference `r`.

Therefore:

```text
reference lifetime
<
data lifetime

✓ safe
```

---

# Lifetime Annotations

## `'a`

Ordinary reference:

```rust
&i32
```

With an explicit lifetime:

```rust
&'a i32
```

Mutable form:

```rust
&'a mut i32
```

Syntax:

```text
&
'a
Type

↓
&'a Type
```

Lifetime names start with:

```text
'
```

and are usually short:

```text
'a
'b
```

`'a` is commonly used first. The Rust Programming Language (2)

---

## Critical Point: An Annotation Does Not Extend a Lifetime

This is essential.

Writing:

```rust
&'a str
```

does not mean:

```text
"make this string live longer"
```

It means:

```text
"this reference participates
in the lifetime relationship 'a"
```

Lifetime annotations **describe relationships**; they do not control how long data exists.

```text
annotation
≠ change lifetime

annotation
= describe lifetime relationships
```

The Rust Programming Language (2)

---

# Lifetimes in Functions

## The Problem with Two Input References

Consider:

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

It returns a reference.

But which one?

```text
sometimes → x
sometimes → y
```

The compiler does not know in advance:

```text
which branch will execute
which argument will be returned
how the lifetime of x relates to the lifetime of y
```

So the signature is insufficient.

---

## Connecting References with `'a`

```rust
fn longest<'a>(
    x: &'a str,
    y: &'a str,
) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Now the contract tells Rust:

```text
x      → lives at least as long as 'a
y      → lives at least as long as 'a
result → lives at least as long as 'a
```

This must be understood correctly.

It **does not mean** the actual lifetimes of `x` and `y` must be completely identical.

For a particular call, `'a` becomes the region where both reference lifetimes overlap.

Practically, for `longest`:

```text
lifetime(result)
=
the shorter of lifetime(x) and lifetime(y)
```

The Rust Programming Language (2)

---

## Why the Shorter Lifetime Is Used

For example:

```rust
let string1 = String::from("long string is long");

{
    let string2 = String::from("xyz");

    let result =
        longest(string1.as_str(), string2.as_str());

    println!("{result}");
}
```

Diagram:

```text
string1 ────────────────────────────┐
                                   │
      string2 ────────────┐         │
                          │         │
      result  ────────────┘         │
```

The result is used only where **both** input references are valid.

This code is safe.

---

## When Using the Result Is Forbidden

```rust
let string1 = String::from("long string is long");

let result;

{
    let string2 = String::from("xyz");

    result =
        longest(string1.as_str(), string2.as_str());
}

println!("{result}");
```

Problem:

```text
string1 ────────────────────────────────┐
                                       │
     string2 ────────┐                  │
                     ↓                  │
                  destroyed             │
                                       │
                         result is used
```

Even if a person can see that with these particular values the function would return `string1`, the contract of `longest` allows either reference to be returned.

Rust therefore uses the shorter lifetime and rejects using `result` after `string2` is destroyed.

---

# A Lifetime Is Part of a Function's Contract

Annotations are in the **signature**, not the body:

```rust
fn longest<'a>(
    x: &'a str,
    y: &'a str,
) -> &'a str
```

Similar to:

```rust
fn f<T>(...)
```

where `T` abstracts a type,

```rust
fn f<'a>(...)
```

abstracts a reference lifetime.

The signature states the relationships the borrow checker may rely on.

```text
types
→ type contract

lifetimes
→ reference contract
```

---

# Not Every Reference Needs the Same Lifetime

Suppose a function always returns only `x`:

```rust
fn longest<'a>(
    x: &'a str,
    y: &str,
) -> &'a str {
    x
}
```

Here:

```text
result is related to x

result is not related to y
```

So only:

```text
x ↔ result
```

needs to share the lifetime relationship; the lifetime of `y` does not need to be included.

Main principle:

> Lifetime annotations should describe the actual reference relationships created by the function.

---

# A Dangling Reference Cannot Be Fixed with an Annotation

Consider:

```rust
fn longest<'a>(
    x: &str,
    y: &str,
) -> &'a str {
    let result =
        String::from("really long string");

    result.as_str()
}
```

This does not work.

Reason:

```text
result
→ created inside the function
→ owns a String
→ destroyed when the function returns

result.as_str()
→ reference to that String
```

The function would try to return:

```text
a reference
↓
to already destroyed data
```

No annotation such as:

```rust
'a
```

can make `result` live longer.

In this situation, the book points to a different solution: **return an owned value**, not a reference.

For example conceptually:

```rust
fn make_string() -> String {
    String::from("...")
}
```

Ownership then moves to the caller.

This is a crucial rule:

```text
lifetime annotation
can describe a safe reference

but

lifetime annotation
cannot make an unsafe reference safe
```

---

# Lifetimes in Structures

A structure can store a reference:

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

Here `'a` is part of the structure's type.

Meaning:

```text
ImportantExcerpt<'a>
cannot outlive
the data
referenced by part
```

The Rust Programming Language (2)

---

Example:

```rust
let novel =
    String::from("Call me Ishmael. Some years ago...");

let first_sentence =
    novel.split('.').next().unwrap();

let i = ImportantExcerpt {
    part: first_sentence,
};
```

Diagram:

```text
novel
  │
  └────→ string data
             ▲
             │
        first_sentence
             ▲
             │
     ImportantExcerpt.part
```

While a structure containing such a reference exists, the original data must remain valid.

---

## Why a Structure Needs a Lifetime

If you conceptually write:

```rust
struct Something {
    value: &str,
}
```

the question is:

```text
how long must the data
referenced by value live?
```

Therefore a structure with a reference field must express this relationship:

```rust
struct Something<'a> {
    value: &'a str,
}
```

Again:

```text
the structure does not own the data

it borrows the data
```

and the lifetime describes that borrowing.

---

# 3. Important Details

# Lifetime Elision

## Why Lifetimes Are Not Written Everywhere

For example, this familiar function compiles:

```rust
fn first_word(s: &str) -> &str {
    ...
}
```

although formally the relationship looks approximately like:

```rust
fn first_word<'a>(
    s: &'a str,
) -> &'a str {
    ...
}
```

The reason is **lifetime elision rules**.

Rust can infer some common lifetime relationships automatically, so programmers do not need to write `'a` in every function. The Rust Programming Language (2)

---

# The Three Lifetime Elision Rules

The compiler applies three rules in sequence.

## Rule 1

Every reference parameter gets its own lifetime.

Given:

```rust
fn foo(x: &i32)
```

the compiler treats it as:

```rust
fn foo<'a>(x: &'a i32)
```

For two references:

```rust
fn foo(x: &i32, y: &i32)
```

it becomes:

```rust
fn foo<'a, 'b>(
    x: &'a i32,
    y: &'b i32,
)
```

So by default:

```text
each input reference
→ its own lifetime
```

---

## Rule 2

If there is **exactly one input lifetime**, it is assigned to all output references.

```rust
fn first_word(s: &str) -> &str
```

after rule 1:

```rust
fn first_word<'a>(
    s: &'a str,
) -> &str
```

after rule 2:

```rust
fn first_word<'a>(
    s: &'a str,
) -> &'a str
```

So `'a` does not need to be written manually here.

---

## Rule 3

If there are several input lifetimes, but one parameter is:

```rust
&self
```

or:

```rust
&mut self
```

then the lifetime of `self` is assigned to output references.

This is why method signatures are usually not overloaded with lifetime annotations.

The Rust Programming Language (2)

---

# Why `longest` Cannot Be Inferred Automatically

Original signature:

```rust
fn longest(
    x: &str,
    y: &str,
) -> &str
```

After rule 1:

```rust
fn longest<'a, 'b>(
    x: &'a str,
    y: &'b str,
) -> &str
```

Rule 2:

```text
does not apply
→ there are two input lifetimes
```

Rule 3:

```text
does not apply
→ this is a function, not a method
→ there is no self
```

The output lifetime remains unknown.

Therefore Rust requires the relationship to be written explicitly:

```rust
fn longest<'a>(
    x: &'a str,
    y: &'a str,
) -> &'a str
```

---

# Lifetimes in Methods

For the structure:

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

the implementation looks like:

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

Here:

```text
impl<'a>
→ declare the lifetime

ImportantExcerpt<'a>
→ apply it to the type
```

For `&self` itself, a separate explicit annotation is often unnecessary because of lifetime elision.

---

## Returning a Reference from `self`

For example:

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(
        &self,
        announcement: &str,
    ) -> &str {
        println!(
            "Attention please: {announcement}"
        );

        self.part
    }
}
```

There are two input references:

```text
&self
&str
```

By the first rule, they receive different lifetimes.

But because:

```text
&self
```

is present, the third rule ties the output lifetime to `self`.

Therefore the return lifetime does not need to be written manually here.

---

# `'static`

A special lifetime:

```rust
'static
```

means that a reference can live **for the entire duration of the program**.

Main example:

```rust
let s: &'static str =
    "I have a static lifetime.";
```

String literals are stored in the program binary and remain available throughout program execution, so they have a `'static` lifetime. The Rust Programming Language (2)

---

## Do Not “Fix” Errors by Adding `'static`

The compiler may sometimes suggest:

```rust
'static
```

when reporting a lifetime error.

But the book explicitly warns not to mechanically replace a lifetime with `'static`.

First ask:

```text
Does this data really exist
for the entire program?
```

If not, the real problem is usually:

```text
a dangling reference
or
an incorrect lifetime relationship
```

rather than the absence of `'static`.

So:

```text
compiler suggests 'static
        ↓
does not mean:
"always add 'static"
```

---

# Lifetimes + Generics + Traits

All three mechanisms can be used together.

Example from the book:

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");

    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Here:

```text
'a
→ generic lifetime

T
→ generic type

T: Display
→ trait bound
```

They are declared together:

```rust
<'a, T>
```

and behavioral constraints on the type can be moved into:

```rust
where
```

---

# 4. What to Remember

Main definition:

```text
lifetime
→ the region where a reference is valid
```

Main goal:

```text
prevent:

reference
↓
data already destroyed
```

That is:

```text
reference lifetime
must not exceed
data lifetime
```

---

Syntax:

```rust
&i32
```

ordinary reference.

```rust
&'a i32
```

reference with lifetime `'a`.

```rust
&'a mut i32
```

mutable reference with lifetime `'a`.

---

Most important rule:

```text
LIFETIME ANNOTATION
DOES NOT extend a value's lifetime

it only describes
relationships between references
```

---

Core function:

```rust
fn longest<'a>(
    x: &'a str,
    y: &'a str,
) -> &'a str
```

Read it as:

```text
the function accepts two references
valid for at least 'a,

and returns a reference
valid for at least 'a
```

At a call site:

```text
'a
≈ common valid region
for both input references

practically
→ the shorter lifetime
```

---

If the result is related to only one argument:

```rust
fn f<'a>(
    x: &'a str,
    y: &str,
) -> &'a str {
    x
}
```

do not artificially tie `y` to `'a`.

---

This is invalid:

```rust
fn f<'a>() -> &'a str {
    let s = String::from("hello");

    s.as_str()
}
```

because:

```text
s is destroyed
when the function returns

↓
the returned reference would dangle
```

The solution in such a situation is:

```text
return an owned value
```

rather than trying to fix the reference with a lifetime annotation.

---

Structure containing a reference:

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

means:

```text
ImportantExcerpt
cannot outlive the data
referenced by part
```

---

Three elision rules:

```text
1. Each input reference
   gets its own lifetime.

2. If there is one input lifetime,
   it is assigned to output references.

3. For a method with &self / &mut self,
   the lifetime of self is assigned
   to output references.
```

If the relationship is still unclear after these rules:

```text
Rust does not guess

↓
an explicit lifetime annotation is required
```

---

Special lifetime:

```rust
'static
```

means:

```text
a reference can live
for the entire program
```

Classic example:

```rust
let text: &'static str = "hello";
```

But `'static` should not be used as a universal fix for borrow-checker errors.

---

Final connection of Chapter 10:

```text
GENERICS
→ which types may be substituted?

TRAITS
→ what behavior must the type provide?

LIFETIMES
→ how are reference lifetimes related?
```

All three are forms of expressing constraints on abstract code:

```text
T
→ unknown type

T: Trait
→ unknown type
  with known behavior

'a
→ unknown concrete lifetime
  with a known relationship to other references
```

And the most important idea about lifetimes:

> **A lifetime does not tell data how long to live. It tells the compiler how reference lifetimes relate to the data so Rust can prove that no dangling references exist.**

**Notes size:** approximately **10–12 thousand characters** including code and diagrams.

**Time to copy into a notebook:** approximately **60–75 minutes**. If shortening, reduce the number of examples, but keep `longest<'a>`, the relationship between the result and the shorter lifetime, structures containing references, the three elision rules, and the meaning of `'static`.
