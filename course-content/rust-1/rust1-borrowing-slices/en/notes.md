# Step 4. Borrowing, References, and Slices

## 1. Topic

In the previous step, the problem was this:

```rust
let s = String::from("hello");
takes_ownership(s);
```

If a `String` is passed to a function by value, ownership moves into the function, and the original variable can no longer be used.

But often a function needs **temporary access to data only**, without becoming its owner. Rust solves this with **references** and the **borrowing** mechanism.

A reference lets you access a value whose owner remains another variable. Unlike an ordinary pointer, Rust guarantees that a reference points to a valid value of its type for as long as the reference exists. The Rust Programming Language (2)

Main model:

```text
ownership
   ↓
the owner is responsible for the data

borrowing
   ↓
other code gets temporary access
without taking ownership
```

---

# 2. Mechanism

## Reference `&T`

Instead of passing a `String`:

```rust
fn calculate_length(s: String) -> usize {
    s.len()
}
```

you can pass a reference:

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}

let s1 = String::from("hello");
let len = calculate_length(&s1);
```

Here:

```text
s1       → owner of the String
&s1      → reference to the String
s        → reference parameter
```

`calculate_length` gets access to the data but **does not get ownership**.

Therefore after the call:

```rust
println!("{s1}");
```

is still valid.

A reference is written as:

```rust
&T
```

for example:

```rust
&String
```

Create a reference:

```rust
&s1
```

When the reference itself goes out of scope, the original `String` is not destroyed because the reference does not own it.

---

## Borrowing

Creating a reference is called **borrowing** in Rust.

```rust
let s = String::from("hello");

let r = &s;
```

Model:

```text
s
│
│ owns
▼
String
▲
│
r = &s
│
borrows
```

That is:

```text
OWNER     → responsible for the data lifetime
REFERENCE → only gets access
```

The main effect is that a function can use a value **without taking it away from the caller**.

---

## References Are Immutable by Default

An ordinary reference:

```rust
&s
```

allows reading data but not changing it.

For example:

```rust
fn change(s: &String) {
    s.push_str(", world");
}
```

does not compile.

Like ordinary variables, references are immutable by default.

Changing data requires a **mutable reference**:

```rust
&mut T
```

---

## Mutable Borrowing `&mut T`

To let a function change the original value:

```rust
fn change(s: &mut String) {
    s.push_str(", world");
}

let mut text = String::from("hello");

change(&mut text);
```

Three things are required:

```text
1. The owner must be mut

let mut text = ...


2. Create a mutable reference

&mut text


3. The function accepts &mut T

fn change(s: &mut String)
```

After the function call, the modified `String` still belongs to `text`.

```text
text — owner
       ↓
&mut text temporarily lets the function
change the data
       ↓
ownership is not transferred
```

---

# 3. Important Details

## The Main Rule of Mutable References

Mutable references have an important restriction:

> While an active mutable reference to a value exists, no other reference to the same data may be active at the same time.

So this is forbidden:

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{r1}, {r2}");
```

Rust forbids two simultaneous mutable borrows.

Model:

```text
             ┌─ &mut r1
data ────────┤
             └─ &mut r2     ✗
```

Allowed:

```text
data ──────── &mut r1       ✓
```

The reason is controlled mutation of data.

---

## Why Multiple `&mut` References Are Forbidden

The book connects this restriction with preventing **data races**.

A data race occurs when all three conditions are true at the same time:

```text
1. Two or more pointers
   access the same data simultaneously.

2. At least one of them writes to the data.

3. There is no mechanism synchronizing access.
```

This can lead to undefined behavior.

Rust prevents this class of problems at compile time: code with conflicting access does not pass the borrow checker.

---

## Multiple `&mut` References Can Be Used Sequentially

The rule does not forbid several mutable references from ever existing; it forbids **overlapping active mutable references**.

For example:

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
    // work with r1
}

let r2 = &mut s;
```

After the scope ends, `r1` no longer exists, so `r2` can be created.

Model:

```text
time →

&mut r1 ███████
               &mut r2 ███████

✓ no overlap
```

---

## Immutable and Mutable References Together

Several immutable references are allowed:

```rust
let s = String::from("hello");

let r1 = &s;
let r2 = &s;

println!("{r1}, {r2}");
```

because they only read the value:

```text
          ┌── &r1 read
data ─────┤
          └── &r2 read

✓
```

But an immutable and a mutable reference cannot be active at the same time:

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;

let r3 = &mut s; // error

println!("{r1}, {r2}, {r3}");
```

Reason:

```text
&r1 expects:
"the data will not change"

&mut r3 means:
"the data may change"
```

These guarantees conflict.

---

## The Key Borrowing Rule

The rules can essentially be reduced to this model:

```text
At one moment in time, you may have:

EITHER

any number of &T
        ↓
read-only access


OR

one &mut T
        ↓
read + write
```

That is:

```text
many readers
     OR
one writer
```

Not both at the same time.

This is one of Rust's most important rules.

---

## A Reference Lives Until Its Last Use

A reference's active lifetime does not necessarily continue until the closing `}`.

The compiler can determine its **last use**.

For example:

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;

println!("{r1}, {r2}");

// r1 and r2 are no longer used

let r3 = &mut s;

println!("{r3}");
```

This is valid.

Formally, `r1` and `r2` are declared in the same block, but their borrows are no longer needed after the first `println!`.

Model:

```text
&r1 █████
&r2 █████
          &mut r3 █████

✓
```

But if `r1` is used later:

```text
&r1 █████████████
       &mut r3 █████

✗ overlap
```

---

## Dangling References

A **dangling reference** is a reference to data that has already been destroyed.

Rust does not allow such references to be created.

Incorrect:

```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
```

What happens conceptually:

```text
dangle()
   ↓
s is created
   ↓
&s is created
   ↓
function ends
   ↓
s is destroyed
   ↓
the returned reference would point
at already freed data
```

Therefore the code does not compile. Rust guarantees that data cannot disappear before a reference to it is finished being used. The Rust Programming Language (2)

In this case, the solution is:

```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```

The `String` itself is moved out, transferring ownership to the caller.

Lifetimes, which formally describe such relationships to the compiler, are covered separately later.

---

# Slices

## What Is a Slice?

A **slice** is a reference to a contiguous sequence of elements in a collection.

A slice:

```text
does not own the data
        +
may refer to only part of a collection,
not necessarily the whole collection
```

So a slice is a kind of reference. The Rust Programming Language (2)

For example, instead of storing the numeric index where the first word ends, it is better to store a reference directly to the corresponding part of the string.

---

## Why Slices Are Needed

Suppose a function calculates the end of the first word and returns:

```rust
usize
```

For example:

```text
"hello world"
       ↑
       5
```

Problem:

```rust
let mut s = String::from("hello world");

let index = 5;

s.clear();
```

`index` is still `5`, but it no longer corresponds to the current contents of the string.

So:

```text
String and index
are not connected by the type system
```

The program has to keep them consistent manually.

A slice solves this problem: the result becomes **a reference directly to the original data**.

---

## String Slice `&str`

A string slice has type:

```rust
&str
```

For example:

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

The range:

```text
[start..end]
```

means:

```text
start is included
end is excluded
```

So:

```rust
&s[0..5]
```

takes bytes:

```text
0 1 2 3 4
h e l l o
```

Internally, a slice stores enough information to determine the start of the region and its length.

```text
String:

h e l l o   w o r l d
0 1 2 3 4 5 6 7 8 9 10
            ↑─────────
            world

slice:
ptr ───────────→ index 6
len = 5
```

---

## Range Shorthand

If a slice starts at `0`:

```rust
&s[0..2]
```

it can be written as:

```rust
&s[..2]
```

If it continues to the end:

```rust
&s[3..s.len()]
```

it can be written as:

```rust
&s[3..]
```

The entire string:

```rust
&s[..]
```

Summary:

```text
&x[a..b]  from a to b, excluding b
&x[..b]   from the beginning to b
&x[a..]   from a to the end
&x[..]    the whole value
```

---

## Important Pitfall: UTF-8

String-slice ranges refer to **byte positions**.

Slice boundaries must fall on valid UTF-8 character boundaries.

If a multi-byte character is cut in the middle, the program panics.

Therefore:

```rust
&s[0..5]
```

must not be interpreted mechanically as “the first five human-readable characters.”

In this part of the book, slice examples use ASCII for simplicity; UTF-8 details are covered later.

---

## A Slice Connects the Result to the Original Data

The first-word function can now return:

```rust
fn first_word(s: &String) -> &str
```

instead of an independent `usize`.

For example:

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
```

The important part of the example is not the search algorithm but the result type:

```text
usize
→ just a number, not connected to String

&str
→ a reference directly to part of String
```

If the first space is at position `5`:

```text
"hello world"

&str → "hello"
```

---

## Why a Slice Is Safer Than an Index

Consider:

```rust
let mut s = String::from("hello world");

let word = first_word(&s);

s.clear();

println!("{word}");
```

This does not compile.

Why?

`word` is an immutable reference to part of `s`.

```text
word
 ↓
&s[..5]
 ↓
immutable borrow of s
```

But:

```rust
s.clear();
```

must modify `s`, so it requires mutable access.

This creates a conflict:

```text
word : &str
      ↓
immutable borrow

s.clear()
      ↓
mutable borrow

✗ cannot coexist
```

So a slice is not just more convenient than an index — it lets the compiler connect the validity of the result to the original data and detect the problem at compile time. The Rust Programming Language (2)

---

## String Literals Are Already `&str`

When we write:

```rust
let s = "Hello, world!";
```

the type of `s` is:

```rust
&str
```

A string literal is a string slice pointing to the corresponding data stored in the program binary.

This also explains its immutability:

```text
string literal
     ↓
&str
     ↓
immutable reference
```

The Rust Programming Language (2)

---

## Prefer `&str` to `&String` in Parameters When Possible

Initially a function might look like:

```rust
fn first_word(s: &String) -> &str
```

But the book improves the signature to:

```rust
fn first_word(s: &str) -> &str
```

This makes the function more general.

You can pass:

```rust
let text = String::from("hello world");

first_word(&text);
first_word(&text[..]);
first_word(&text[0..5]);
```

and also a string literal:

```rust
let text = "hello world";

first_word(text);
```

Therefore, for a function that only needs to read string data, `&str` provides a more flexible API than `&String`. The Rust Programming Language (2)

---

## Slices Are Not Limited to Strings

For an array:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

`slice` contains:

```text
[2, 3]
```

and its type is:

```rust
&[i32]
```

General form:

```text
&[T]
```

That is:

```text
&str    → string slice
&[T]    → slice of elements of type T
```

An array slice, like a string slice, does not own the elements; it refers to a contiguous part of a collection. The Rust Programming Language (2)

---

# 4. What to Remember

Main distinction:

```text
String
→ owns string data

&String
→ reference to the whole String
→ does not own it

&str
→ string slice
→ reference to string data
→ may refer to only part of a string

&mut String
→ mutable reference
→ allows modifying the String
```

Main borrowing rules:

```text
At one moment in time:

1. You may have several &T references.

OR

2. You may have one &mut T reference.

But you may not simultaneously have:
&T + &mut T
or
&mut T + &mut T
to the same data.
```

Reason:

```text
many readers
→ safe

one writer
→ safe

reader + writer
→ conflict

multiple writers
→ conflict
```

Remember:

```rust
&T
```

→ immutable borrow.

```rust
&mut T
```

→ mutable borrow.

```rust
let r = &value;
```

→ ownership is not transferred.

```rust
let r = &mut value;
```

→ temporary exclusive access for mutation.

References must not outlive their data:

```text
reference lifetime
cannot be longer than
the lifetime of the referenced data
```

Therefore Rust prevents dangling references.

---

## Minimal Reference Model

```text
OWNER
  │
  ▼
DATA
  ▲
  │
&T
borrow
```

Reference disappears:

```text
DATA remains
```

Owner disappears:

```text
DATA is released
```

But the compiler will not let the owner destroy the data while an active reference still needs to use it.

---

## Minimal Slice Model

```text
COLLECTION

[ A B C D E F ]
    ↑─────↑
      slice
```

A slice:

```text
→ does not copy elements
→ does not own them
→ points to a contiguous part
```

For strings:

```rust
&str
```

For other sequences:

```rust
&[T]
```

---

## Practical Rule for Functions

If a function should **take ownership of the value**:

```rust
fn f(s: String)
```

If it should **only read string data**:

```rust
fn f(s: &str)
```

If it should **modify an existing String without taking ownership**:

```rust
fn f(s: &mut String)
```

The signature often already tells you the function's relationship to the data:

```text
String       → ownership
&str / &T    → borrowing for reading
&mut T       → borrowing for mutation
```

---

### Final Connection to Ownership

```text
OWNERSHIP
    ↓
who is responsible for the data?

BORROWING
    ↓
who may temporarily use it?

&T
    ↓
shared reading

&mut T
    ↓
exclusive mutation

SLICE
    ↓
reference to part of the data
```

The main idea of Chapter 4 is therefore:

> **Data has an owner, but ownership does not have to be transferred merely to use the data. It can be borrowed temporarily, while the compiler ensures that references do not conflict and do not outlive the data itself.**

The book concludes that ownership, borrowing, and slices together provide memory safety at compile time. The Rust Programming Language (2)

**Size:** approximately **10–12 thousand characters** including code and diagrams.
**Time to copy into a notebook:** approximately **60–75 minutes**. This is one of Rust's central topics; shortening the `&` / `&mut` rules and their relationship with `&str` further is not recommended.
