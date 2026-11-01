# Step 3. Ownership and Memory

## 1. Topic

**Ownership** is the system of rules Rust uses to manage memory.

In languages with garbage collection, unused memory is tracked while the program runs. In other languages, the programmer may free it manually. Rust uses a different approach: **ownership rules are checked by the compiler**, and memory is released automatically when the owner of the data no longer exists.

The ownership system does not require a garbage collector and, by itself, does not add runtime ownership checks.

Three main rules:

```text
1. Every value has an owner.

2. At any given time, a value
   can have only one owner.

3. When the owner goes out of scope,
   the value is dropped.
```

This is the foundation from which `move`, borrowing, references, lifetimes, and some concurrency rules follow.

---

# 2. Mechanism

## Stack and Heap

To explain ownership, the book first separates memory into the **stack** and the **heap**.

### Stack

The stack follows the principle:

```text
last in → first out
LIFO
```

Data is added to the top of the stack and removed from the top.

To be stored directly on the stack, the size of the data must be known at compile time.

Conceptually:

```text
Stack

┌─────────┐
│ value C │ ← top
├─────────┤
│ value B │
├─────────┤
│ value A │
└─────────┘
```

Adding and removing stack data is fast because no search for a free region is required.

When a function is called, its parameters and local variables are placed on the stack. When the function finishes, they are removed.

---

### Heap

If the size of data is not known in advance or can change, the data may be stored on the heap.

Allocation works roughly like this:

```text
request memory
    ↓
find a free region
    ↓
allocate the region
    ↓
return a pointer to it
```

The pointer itself has a fixed size and can be stored on the stack.

This gives a model such as:

```text
STACK                         HEAP

┌───────────────┐             ┌───────────────┐
│ ptr ──────────┼────────────→│ data          │
│ length        │             │ ...           │
│ capacity      │             │ ...           │
└───────────────┘             └───────────────┘
```

Allocating heap memory is more expensive than placing a value on the stack because a suitable free region must first be found.

Accessing heap data also requires following a pointer.

The main idea for this chapter is:

> ownership primarily answers the question: **who is responsible for the data and who must release its memory when it is no longer needed**.

---

## Scope and the Owner

A variable is valid from the point it is declared until the end of its scope:

```rust
{
    let s = "hello";

    // s is available here
}

// s is no longer available
```

For simple values this looks familiar.

But for heap data, the end of the scope has an additional meaning: Rust must release the allocated memory.

---

## `String` as an Ownership Example

The book uses `String` because its contents are stored on the heap.

A string literal:

```rust
let s = "hello";
```

is known at compile time.

A `String`, however, can contain data whose size is not known at compile time:

```rust
let mut s = String::from("hello");

s.push_str(", world!");
```

To allow the text to grow, `String` uses heap memory.

When executing:

```rust
String::from("hello")
```

memory is allocated at runtime.

This creates an important memory-management problem:

```text
who allocated the memory?
when is it no longer needed?
who must release it?
```

Rust assigns this responsibility to the owner.

---

## Automatic Release: `drop`

When the owner leaves its scope, Rust automatically calls a special function:

```text
drop
```

Simplified:

```rust
{
    let s = String::from("hello");

    // s owns the String
}

// end of scope
// String memory is released automatically
```

Model:

```text
String::from(...)
      ↓
allocate memory
      ↓
variable becomes the owner
      ↓
...
      ↓
owner leaves the scope
      ↓
drop
      ↓
memory is returned
```

Rust therefore aims to ensure this correspondence:

```text
one memory allocation
        ↕
one release
```

There is no need to decide manually where to call `free`, and there is no need for a garbage collector that later searches for unused objects.

---

# 3. Important Details

## How `String` Is Structured

A `String` has three pieces of metadata:

```text
ptr       → pointer to data
len       → current length
capacity  → allocated capacity
```

This metadata is stored on the stack.

The characters themselves are stored on the heap.

Conceptually:

```text
s
STACK                    HEAP

ptr ───────────────────→ h e l l o
len      5
capacity 5
```

`len` is the number of bytes currently used by the contents.

`capacity` is the number of bytes of memory allocated for the `String`.

For understanding ownership, it is enough to know that `String` contains stack metadata pointing to heap data.

---

## Move — Transferring Ownership

Consider:

```rust
let s1 = String::from("hello");
let s2 = s1;
```

Rust **does not automatically create a fully independent copy of the `hello` data**.

The stack parts of `String` are copied:

```text
ptr
len
capacity
```

But there is still only one heap allocation.

Without ownership rules the state could look like:

```text
s1 ─┐
    ├────→ "hello"
s2 ─┘
```

If both variables were considered owners, both would try to release the same memory when leaving scope.

That would cause:

```text
double free
```

— releasing the same memory region twice.

Rust prevents this differently:

```rust
let s2 = s1;
```

after this operation:

```text
s1 → can no longer be used
s2 → new owner
```

An ownership **move** has occurred:

```text
before:

s1 ─────→ heap data


let s2 = s1;


after:

s1   invalid

s2 ─────→ heap data
```

This attempt:

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{s1}");
```

does not compile.

The reason is not that the data disappeared. The data now belongs to `s2`.

---

## Why Rust Does Not Deep-Copy Automatically

When executing:

```rust
let s2 = s1;
```

Rust could create a second copy of the heap data.

But then a simple assignment could potentially perform an expensive operation:

```text
large data
      ↓
copy the entire allocation
```

Instead, Rust uses a `move`.

This gives an important property:

> Rust does not automatically perform expensive deep copies of data.

If the code does not explicitly show complex data being copied, an expensive copy should not unexpectedly happen just because of ordinary assignment.

---

## `clone()` — Explicit Deep Copy

If two independent copies are really required:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```

Now:

```text
s1 ─────→ "hello"

s2 ─────→ "hello"
```

The heap contains independent data.

Both variables remain valid:

```rust
println!("{s1}");
println!("{s2}");
```

`clone()` is also important as a visual signal:

```text
clone()
   ↓
data is actually being copied here
   ↓
the operation may be expensive
```

Therefore:

```rust
let s2 = s1;
```

and:

```rust
let s2 = s1.clone();
```

have fundamentally different meanings.

---

## `Copy` — Cheap Copying

Now consider:

```rust
let x = 5;
let y = x;

println!("{x}");
println!("{y}");
```

This works.

Why did `x` not become invalid?

Because simple types like this implement the special trait:

```text
Copy
```

For `Copy` types, assignment means an ordinary cheap copy of the value.

```text
let y = x;

Copy type:
x → 5
y → 5
```

Not:

```text
x → invalid
y → 5
```

Examples of types that implement `Copy`:

```text
integers: i32, u32, ...
bool
f32, f64
char
```

A tuple is also `Copy` only when **all of its elements** are `Copy`.

For example:

```text
(i32, i32)    → Copy

(i32, String) → not Copy
```

---

## `Copy` and `Drop`

If a type requires special cleanup logic through `Drop`, Rust does not allow it to be `Copy` at the same time.

The idea is simple:

```text
Copy
→ the value can simply be duplicated

Drop
→ destroying the value requires a special action
```

These two models would conflict.

---

## Move vs Copy — the Main Difference

A useful mental model:

```text
Assignment:

let b = a;

        │
        ▼
Does the type implement Copy?
        │
   ┌────┴─────┐
   │          │
  yes         no
   │          │
   ▼          ▼
COPY         MOVE

a valid      a invalid
b valid      b owner
```

For `String`:

```rust
let s2 = s1;
```

→ `move`.

For `i32`:

```rust
let y = x;
```

→ `copy`.

---

## Ownership When Passing Values to Functions

Passing a value to a function follows the same rules as assignment.

### A Non-`Copy` Type

```rust
fn takes_ownership(s: String) {
    println!("{s}");
}

let s = String::from("hello");

takes_ownership(s);
```

What happens:

```text
main owns s
     ↓
function call
     ↓
ownership moves to the function parameter
     ↓
s in main can no longer be used
```

When the function finishes, its parameter goes out of scope and the data is released.

---

### A `Copy` Type

```rust
fn makes_copy(x: i32) {
    println!("{x}");
}

let x = 5;

makes_copy(x);

println!("{x}");
```

This works.

Because `i32` implements `Copy`:

```text
x in main
  ↓
copied
  ↓
function parameter
```

The original `x` remains available.

---

## Functions Can Return Ownership

A returned value can also transfer ownership.

```rust
fn create_string() -> String {
    let s = String::from("hello");
    s
}
```

With:

```rust
let s1 = create_string();
```

this happens:

```text
s inside the function
       ↓
return
       ↓
ownership moves outward
       ↓
s1 becomes the owner
```

The value is therefore not destroyed when the function ends because ownership was transferred to the caller.

---

## A Function Can Take and Return the Same Ownership

```rust
fn take_and_return(s: String) -> String {
    s
}

let s1 = String::from("hello");
let s2 = take_and_return(s1);
```

Sequence:

```text
s1
 ↓ move
parameter s
 ↓ move on return
s2
```

After that:

```text
s1 → invalid
s2 → owner
```

---

## General Model of a Moving Value

For a type such as `String`, ownership follows the value:

```text
creation
  ↓
owner A
  ↓ assignment / function argument
owner B
  ↓ return
owner C
  ↓
end of scope
  ↓
drop
```

At any moment, only the current owner releases the memory.

This prevents several ordinary variables from independently treating the same memory as their own.

---

## Why Passing Ownership Back and Forth Is Inconvenient

If a function only wants to inspect a `String`, a scheme like:

```text
String → function → return String back
```

would be extremely inconvenient.

For example, technically you could return the original string together with a result:

```rust
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
```

But constantly moving ownership into a function and back out is awkward to write.

Rust solves exactly this problem with **references and borrowing**.

That is the next step.

---

# 4. What to Remember

Main model of the entire topic:

```text
VALUE
  ↓
has one owner
  ↓
the owner can change through MOVE
  ↓
current owner leaves scope
  ↓
DROP
  ↓
resource is released
```

You must remember the three rules:

```text
1. Every value has an owner.

2. An ordinary value has only one owner
   at a time.

3. Owner leaves scope
   → value is destroyed.
```

Memory model:

```text
STACK
→ fixed-size data
→ fast push/pop
→ may store String ptr/len/capacity

HEAP
→ dynamically sized data
→ memory must be allocated
→ accessed through a pointer
→ contains String contents
```

For `String`:

```text
STACK                    HEAP

ptr ───────────────────→ data
len
capacity
```

Key difference:

```rust
let s2 = s1;
```

for `String` means:

```text
MOVE
s1 invalid
s2 owner
```

while:

```rust
let s2 = s1.clone();
```

means:

```text
deep copy
s1 valid
s2 valid
```

For `Copy` types:

```rust
let y = x;
```

means:

```text
copy
x valid
y valid
```

Examples of `Copy`:

```text
integers
floats
bool
char
tuple, if all elements are Copy
```

And the main rule for functions:

```text
passing an argument
works like assignment

non-Copy → move
Copy     → copy
```

Returning a value can also transfer ownership:

```text
function
   ↓ return
ownership moves to the caller
```

### Final Model

When you see code such as:

```rust
let b = a;
```

the first question is:

```text
What is the type of a?
```

If it is `Copy`:

```text
a remains valid
b receives a copy
```

If it is a value such as `String`:

```text
ownership moves to b
a becomes invalid
```

This is why a variable can no longer be treated as merely “a name containing a value.” In Rust it also matters:

```text
who currently owns this value?
```

The next step is **4. Borrowing, References, and Slices**. It solves the main limitation of pure moves: how to give a function or another part of the program access to data **without transferring ownership**.

**Size:** approximately **9–11 thousand characters** including code and diagrams.
**Time to copy into a notebook:** approximately **55–70 minutes**. It is better not to shorten this topic much further: `move`, `Copy`, `clone`, `drop`, and ownership transfer through functions are foundational to the rest of Rust.
