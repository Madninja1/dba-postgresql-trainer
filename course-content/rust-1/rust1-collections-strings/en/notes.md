# Step 7. Collections and Strings

## 1. Topic

Standard-library collections let you store **multiple values**, and their size may be unknown at compile time and change while the program runs. Unlike arrays and tuples, the data of the collections covered here is stored on the heap. The Rust Programming Language (2)

This chapter focuses on three main types:

```text
Vec<T>
→ sequence of elements of one type

String
→ mutable owned UTF-8 string

HashMap<K, V>
→ key → value pairs
```

The main connection with previous topics:

```text
collections
   ↓
store data
   ↓
follow the ordinary rules of
ownership + borrowing
```

---

# 2. Mechanism

## `Vec<T>` — Dynamic List

A vector stores a sequence of elements of **one type**:

```rust
let v = vec![1, 2, 3];
```

Type:

```text
Vec<i32>
```

Unlike an array, the number of vector elements can change.

Empty vector:

```rust
let v: Vec<i32> = Vec::new();
```

Here the type must be specified explicitly because the compiler cannot infer the element type from an empty vector.

If initial values are present:

```rust
let v = vec![1, 2, 3];
```

the type is inferred automatically. The Rust Programming Language (2)

---

## Adding Elements

Mutation requires `mut`:

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
```

After the insertions, Rust can infer:

```text
Vec<i32>
```

All elements of one `Vec<T>` must have the same type.

---

## Accessing an Element

There are two main ways.

### Indexing

```rust
let v = vec![1, 2, 3, 4, 5];

let third = &v[2];
```

Result:

```text
&i32
```

Indexing starts at `0`.

If the index is invalid:

```rust
&v[100]
```

the program panics.

---

### `get`

```rust
let third = v.get(2);
```

Result:

```text
Option<&T>
```

That is:

```text
element exists
→ Some(&value)

element does not exist
→ None
```

For example:

```rust
match v.get(100) {
    Some(value) => println!("{value}"),
    None => println!("No element"),
}
```

Difference:

```text
v[index]
→ invalid index = panic

v.get(index)
→ invalid index = None
```

`get` is appropriate when the absence of an element is a normal possible outcome. The Rust Programming Language (2)

---

## Vector and Borrowing

This code is forbidden:

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("{first}");
```

The reason is more than a syntactic rule.

The elements of `Vec<T>` are stored contiguously. During `push`, the current allocation may no longer have enough room.

The vector may then:

```text
allocate a new region
        ↓
move the elements there
        ↓
release the old region
```

The old reference:

```text
first → old memory region
```

would become invalid.

Therefore the borrow checker prevents this kind of vector mutation while a reference to one of its elements is active. The Rust Programming Language (2)

This is an important practical connection:

```text
reference to a Vec element
        ↓
may restrict mutation of the Vec itself
```

---

## Iterating Over a `Vec`

Reading:

```rust
let v = vec![100, 32, 57];

for i in &v {
    println!("{i}");
}
```

The elements are borrowed through immutable references.

Mutation:

```rust
let mut v = vec![100, 32, 57];

for i in &mut v {
    *i += 50;
}
```

`i` is a mutable reference, so dereferencing is used to change the value:

```rust
*i
```

During such a loop, elements cannot simultaneously be inserted into or removed from the vector because the loop already borrows it. The Rust Programming Language (2)

---

## Different Types in One `Vec`

Directly:

```text
Vec<T>
→ one T
```

But if the possible set of types is known in advance, they can be combined with an `enum`:

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

In fact, all elements have one type:

```text
SpreadsheetCell
```

while the different data lives inside its variants.

The variants can then be handled with `match`.

This applies the already studied:

```text
enum + match
```

to create a type-safe heterogeneous collection whose possible variants are known in advance. The Rust Programming Language (2)

---

## Dropping a `Vec`

When a vector goes out of scope:

```rust
{
    let v = vec![1, 2, 3];
}
```

both are dropped:

```text
the Vec itself
+
its elements
```

So ordinary ownership rules continue to apply to collections. The Rust Programming Language (2)

---

# `String` and `&str`

## Two Main String Types

The language core contains:

```text
str
```

usually used in borrowed form:

```rust
&str
```

The standard library provides:

```rust
String
```

`String`:

```text
owns its data
mutable
can grow
UTF-8
```

`&str`:

```text
borrowed string slice
UTF-8
```

String literals:

```rust
"hello"
```

have the form `&str`.

The book describes `String` as a wrapper around a sequence of bytes with additional guarantees and operations for working with text. The Rust Programming Language (2)

---

## Creating a `String`

Empty:

```rust
let mut s = String::new();
```

From `&str`:

```rust
let s = String::from("hello");
```

or:

```rust
let s = "hello".to_string();
```

In this case, `String::from` and `to_string` produce equivalent results. The Rust Programming Language (2)

---

## Adding Text

### `push_str`

Adds a string slice:

```rust
let mut s = String::from("foo");

s.push_str("bar");
```

Result:

```text
foobar
```

`push_str` accepts `&str`, so it does not need to take ownership of the supplied string.

For example:

```rust
let mut s1 = String::from("foo");
let s2 = "bar";

s1.push_str(s2);

println!("{s2}");
```

`s2` remains available. The Rust Programming Language (2)

---

### `push`

Adds one `char`:

```rust
let mut s = String::from("lo");

s.push('l');
```

Result:

```text
lol
```

Difference:

```text
push_str(&str)
→ add a string slice

push(char)
→ add one char
```

---

# Concatenating Strings

## `+` Operator

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");

let s3 = s1 + &s2;
```

After the operation:

```text
s1 → no longer valid
s2 → remains valid
s3 → owns the result
```

The reason can be seen from the approximate signature of the mechanism being called:

```rust
fn add(self, s: &str) -> String
```

First argument:

```text
self
→ ownership is passed
```

Second:

```text
&str
→ only borrowed
```

Therefore:

```rust
s1 + &s2
```

moves `s1` but not `s2`.

The compiler can coerce:

```text
&String → &str
```

for the second argument. The Rust Programming Language (2)

---

## `format!`

For more complex concatenation:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}");
```

produces:

```text
tic-tac-toe
```

`format!`:

```text
returns String
+
uses references to its arguments
+
does not take their ownership
```

For combining several values it is often easier to read than a chain of `+` operations. The Rust Programming Language (2)

---

# The Main Thing About UTF-8

## `String` Is Not an Array of Characters

This is one of the central details of the chapter.

Rust does not allow:

```rust
let s = String::from("hello");

let c = s[0]; // error
```

The reason is that `String` is stored as UTF-8, where one perceived character does not necessarily correspond to one byte. The Rust Programming Language (2)

For example, the book shows:

```rust
"Hola"
```

has length:

```text
4 bytes
```

while:

```rust
"Здравствуйте"
```

occupies:

```text
24 bytes
```

even though the word visually contains 12 letters.

Therefore:

```text
index 0
```

cannot unambiguously mean “the first character.”

---

## Three Levels of Text Representation

The book identifies three ways to view a UTF-8 string:

```text
1. bytes
   ↓
   actual u8 values in memory

2. Unicode scalar values
   ↓
   Rust char

3. grapheme clusters
   ↓
   what a user may perceive as letters
```

These concepts do not always correspond one-to-one.

So:

```text
1 byte
≠ necessarily 1 char
≠ necessarily 1 visible character
```

The Rust Programming Language (2)

---

## Why `String[index]` Is Forbidden

There are two main reasons.

First:

```text
it is unclear what to return:

byte?
char?
grapheme?
```

Second — performance.

A programmer generally expects indexing to be:

```text
O(1)
```

But locating an arbitrary Unicode character would require scanning the string and analyzing UTF-8 boundaries.

Therefore Rust requires the programmer to state explicitly what should be retrieved. The Rust Programming Language (2)

---

# String Slices

You can take a byte range:

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

The example produces:

```text
"Зд"
```

because these characters each occupy two bytes.

But:

```rust
&hello[0..1]
```

lands inside a multibyte character and panics.

Therefore:

```text
a string slice uses byte boundaries
```

and the range must follow valid UTF-8 boundaries. The Rust Programming Language (2)

This is one of the main pitfalls of `String`.

---

## `chars()` and `bytes()`

If Unicode scalar values are needed:

```rust
for c in "Зд".chars() {
    println!("{c}");
}
```

we get values of type:

```text
char
```

If the actual bytes are needed:

```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```

we get:

```text
208
151
208
180
```

So when working with strings, choose explicitly:

```text
need Unicode characters?
→ chars()

need bytes?
→ bytes()
```

The standard library does not directly provide grapheme-cluster segmentation; the book notes that separate libraries exist for this. The Rust Programming Language (2)

---

# `HashMap<K, V>`

`HashMap` stores mappings:

```text
key → value
```

For example:

```text
"Blue"   → 10
"Yellow" → 50
```

Unlike `Vec`, where data is normally found by numeric position, `HashMap` lets you retrieve a value by key.

All keys must have one type `K`, and all values one type `V`. The data is stored on the heap. The Rust Programming Language (2)

---

## Creating a `HashMap`

`HashMap` is not in the prelude, so it must be imported:

```rust
use std::collections::HashMap;
```

Create one:

```rust
let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

This produces a type like:

```text
HashMap<String, i32>
```

---

## Reading

```rust
let team = String::from("Blue");

let score = scores.get(&team);
```

`get` returns:

```text
Option<&V>
```

That is:

```text
key found
→ Some(&value)

key not found
→ None
```

The book also uses:

```rust
scores.get(&team).copied().unwrap_or(0)
```

to obtain an `i32` value and use `0` if the key is absent. The Rust Programming Language (2)

---

## Iterating Over a `HashMap`

```rust
for (key, value) in &scores {
    println!("{key}: {value}");
}
```

Important:

```text
HashMap iteration order
is not guaranteed to match insertion order
```

The pairs may be printed in an arbitrary order in the book's examples. The Rust Programming Language (2)

---

# `HashMap` and Ownership

When inserting `Copy` values:

```rust
scores.insert(key, 10);
```

such as `i32`, the value is copied.

For owning types:

```rust
let key = String::from("Favorite color");
let value = String::from("Blue");

map.insert(key, value);
```

both `String`s are moved into the map:

```text
key ─── move ───→ HashMap
value ─ move ───→ HashMap
```

The original variables can no longer be used afterward.

If the `HashMap` stores references instead, ownership of the referenced data does not move, but that data must remain valid long enough. The Rust Programming Language (2)

---

# Updating a `HashMap`

## Overwriting

If a value is inserted with an existing key:

```rust
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);
```

the result is:

```text
"Blue" → 25
```

The new value replaces the old one. The Rust Programming Language (2)

---

## Insert Only If the Key Is Missing

For this, the API provides:

```rust
entry(...)
```

Example:

```rust
scores
    .entry(String::from("Yellow"))
    .or_insert(50);
```

Meaning:

```text
key exists
→ keep the current value

key is absent
→ insert 50
```

`or_insert` returns a mutable reference to the value:

```text
&mut V
```

The Rust Programming Language (2)

---

## Update a Value Based on Its Old Value

This enables patterns such as:

```rust
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```

Mechanism:

```text
entry(word)
    ↓
key exists?
 ┌──┴──┐
yes    no
│      │
value  insert 0
 └──┬──┘
    ↓
&mut value
    ↓
*count += 1
```

The result is a count of how many times each word appears. The Rust Programming Language (2)

This is one of the most useful `HashMap` patterns.

---

# 3. Important Details

## Collections Do Not Bypass Borrowing

All three types still follow the previous rules:

```text
ownership
borrowing
move
Copy
drop
```

Especially for `Vec`:

```text
&v[index]
+
change the size of v
→ possible memory relocation
→ Rust forbids the conflict
```

For `HashMap`:

```text
insert(String, String)
→ ownership may move into the map
```

---

## `Vec<T>` Contains One Type

```text
Vec<i32>      ✓
Vec<String>   ✓
```

For a known set of different data shapes:

```text
enum
+
Vec<Enum>
```

---

## `String` Length Is About Bytes

Do not assume:

```text
len() = number of human-readable letters
```

That is false for UTF-8.

Model:

```text
String
→ UTF-8 bytes
```

This is why ordinary numeric indexing cannot be used to get a character.

---

## A String Range Can Panic

```rust
&s[a..b]
```

works with byte positions.

If `a` or `b` cuts through a UTF-8 character:

```text
panic
```

For traversing text, choose explicitly:

```rust
chars()
bytes()
```

---

## `+` Changes Ownership

```rust
let s3 = s1 + &s2;
```

Afterward:

```text
s1 invalid
s2 valid
s3 valid
```

If you do not want to transfer ownership of the original strings and the combination is complex:

```rust
format!(...)
```

is shown in the book as a more readable option.

---

## `HashMap::get`

Returns:

```rust
Option<&V>
```

This connects collections back to the already studied `Option<T>`:

```text
retrieving data may result in
"value exists / value is absent"
```

---

## `entry`

Remember the combination:

```rust
map.entry(key).or_insert(default)
```

It is used for two typical scenarios:

```text
insert default
if the key is absent

or

obtain &mut V
and update the existing value
```

---

## Hashing

The book notes that the default hash function used by `HashMap` is selected with protection against a certain class of DoS attacks in mind, which trades some performance for security. If profiling shows hashing is a bottleneck, the hasher can be replaced. The Rust Programming Language (2)

For basic Rust ownership knowledge, this is enough; `BuildHasher` details do not need to be memorized here.

---

# 4. What to Remember

Main model:

```text
Vec<T>
→ list
→ one type T
→ variable size

String
→ owned UTF-8 text
→ mutable
→ does not support String[index]

HashMap<K, V>
→ key → value
→ one key type K
→ one value type V
```

Minimum for `Vec`:

```rust
let mut v = vec![1, 2, 3];

v.push(4);

let a = &v[0];
let b = v.get(0);
```

Difference:

```text
&v[index]
→ &T
→ invalid index = panic

v.get(index)
→ Option<&T>
→ invalid index = None
```

Iteration:

```rust
for x in &v {
    ...
}

for x in &mut v {
    *x += 1;
}
```

---

Minimum for strings:

```text
String
→ owned

&str
→ borrowed
```

Creation:

```rust
String::new()
String::from("text")
"text".to_string()
```

Appending:

```rust
s.push_str("text");
s.push('x');
```

Concatenation:

```rust
let c = a + &b;
```

remember:

```text
a moved
b borrowed
```

or:

```rust
let c = format!("{a}{b}");
```

---

Main string rule:

```text
String ≠ array of char
```

Internal model:

```text
UTF-8 bytes
     ↓
Unicode scalar values
     ↓
grapheme clusters
```

They do not have to correspond one-to-one.

Therefore:

```rust
string[0]
```

does not work.

Use:

```rust
string.chars()
string.bytes()
```

or carefully:

```rust
&string[a..b]
```

only at valid UTF-8 boundaries.

---

Minimum for `HashMap`:

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

map.insert(key, value);

let result = map.get(&key);
```

`get`:

```text
Option<&V>
```

Update:

```rust
map.insert(key, new_value);
```

→ overwrite.

```rust
map.entry(key).or_insert(default);
```

→ insert only if absent.

Typical counter:

```rust
let count = map.entry(key).or_insert(0);
*count += 1;
```

---

The most important connection in the chapter:

```text
COLLECTIONS
     ↓
store a lot of data
     ↓
but ownership does not disappear

Vec
→ references to elements are tied to the Vec itself

String
→ operations may borrow or move ownership

HashMap
→ inserted Strings may become owned by the map
```

And another useful model:

```text
Vec<T>        → access by position
HashMap<K,V>  → access by key
String        → UTF-8 text, not an indexable character array
```

**Notes size:** approximately **10–12 thousand characters** including code.

**Time to copy into a notebook:** approximately **55–70 minutes**. If shortening, secondary examples can be removed first, but keep `Vec::get`, the conflict between `push` and a reference to an element, the UTF-8 model, `String + &str`, and `HashMap::entry().or_insert()`.
