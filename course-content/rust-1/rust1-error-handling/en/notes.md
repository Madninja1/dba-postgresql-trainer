# Step 8. Errors

## 1. Topic

Rust divides errors into two main categories:

```text
recoverable
→ the error can be handled and execution can continue

unrecoverable
→ continuing execution is considered incorrect
```

Different mechanisms are used for them:

```text
recoverable error
→ Result<T, E>

unrecoverable error
→ panic!
```

Rust does not use exceptions in the form common in many other languages: expected failures are represented as ordinary `Result` values that the calling code must handle. The Rust Programming Language (2)

Main model:

```text
an operation can fail
            ↓
     is the failure expected?
        ┌───┴───┐
       yes      no
       ↓        ↓
    Result    panic!
```

---

# 2. Mechanism

## `panic!` — Stopping the Program

Explicit panic:

```rust
panic!("crash and burn");
```

A panic can also occur automatically, for example when indexing past the end of a vector:

```rust
let v = vec![1, 2, 3];

v[99];
```

In that case Rust does not try to return data from memory outside the structure; it stops execution.

By default, on panic Rust:

```text
prints a message
       ↓
unwinds the call stack
       ↓
cleans up data
       ↓
terminates the program
```

The book also describes an alternative mode — terminating immediately without stack unwinding:

```toml
[profile.release]
panic = 'abort'
```

This reduces the work done during a panic and can reduce the size of the final binary.

---

## Backtrace

To find the source of a panic, you can use:

```bash
RUST_BACKTRACE=1 cargo run
```

A backtrace shows the chain of function calls that led to the panic.

When looking for the cause, the book recommends following the trace until the first location that belongs to your own source code.

An informative backtrace requires debug symbols; ordinary `cargo build` and `cargo run` without `--release` include them by default.

Model:

```text
panic
 ↓
stack backtrace
 ↓
call chain
 ↓
first relevant location in your code
```

---

# `Result<T, E>`

For an error that calling code can handle, Rust uses:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Where:

```text
T
→ successful result type

E
→ error type
```

For example:

```rust
use std::fs::File;

let result = File::open("hello.txt");
```

`File::open` can return:

```text
Ok(File)
→ the file was opened successfully

Err(io::Error)
→ an error occurred
```

So the error is part of the result type rather than a hidden side effect.

---

## Handling with `match`

Full handling:

```rust
use std::fs::File;

let result = File::open("hello.txt");

let file = match result {
    Ok(file) => file,
    Err(error) => panic!("Problem opening the file: {error:?}"),
};
```

Mechanism:

```text
Result<T, E>
     ↓
   match
 ┌────┴────┐
Ok(T)     Err(E)
 ↓          ↓
T       handling
```

`Ok` and `Err`, like `Result`, are in the prelude, so you normally do not need to write:

```rust
Result::Ok(...)
Result::Err(...)
```

---

## Different Errors — Different Handling

The reason for an error can affect program behavior.

In the book's example:

```text
File::open
    ↓
error?
    ↓
file not found?
 ┌──────┴──────┐
yes             no
↓               ↓
create file    panic
```

For `io::Error`, the code uses:

```rust
error.kind()
```

and checks:

```rust
ErrorKind::NotFound
```

Simplified:

```rust
match File::open("hello.txt") {
    Ok(file) => file,

    Err(error) => match error.kind() {
        ErrorKind::NotFound => {
            // try to create the file
        }

        _ => {
            panic!("Problem opening file");
        }
    },
}
```

The important point is not the number of nested `match` expressions, but the principle:

> `Err` contains data that lets the program determine the reason for failure and choose an appropriate response.

---

# `unwrap()`

`Result` has a shortcut:

```rust
let file = File::open("hello.txt").unwrap();
```

Behavior:

```text
Ok(value)
→ return value

Err(error)
→ panic!
```

Conceptually:

```rust
result.unwrap()
```

roughly replaces the simplest form of:

```rust
match result {
    Ok(value) => value,
    Err(error) => panic!(...),
}
```

`unwrap` is not full error handling: on `Err`, the program simply panics.

---

# `expect()`

Similar to `unwrap`, but lets you provide your own message:

```rust
let file = File::open("hello.txt")
    .expect("hello.txt should be included in this project");
```

Behavior:

```text
Ok(value)
→ value

Err(error)
→ panic + our message
```

The book prefers `expect` to `unwrap` when panicking is genuinely acceptable because the message explains **why the operation was expected to succeed**.

```text
unwrap
→ "I expect success here"

expect("reason")
→ "I expect success here for this reason"
```

---

# `unwrap_or_else`

The chapter also shows:

```rust
result.unwrap_or_else(...)
```

Its idea:

```text
Ok(value)
→ return value

Err(error)
→ execute the supplied handling code
```

This can replace some nested `match` expressions with shorter code.

Closures used with this method are covered later in the book, so for now the principle is enough.

---

# Propagating an Error

A function does not have to decide what to do with an error itself.

It can return the error to its caller:

```rust
fn read_username_from_file()
    -> Result<String, std::io::Error>
{
    ...
}
```

On success:

```text
Ok(String)
```

on failure:

```text
Err(io::Error)
```

Manual example:

```rust
let file = match File::open("hello.txt") {
    Ok(file) => file,
    Err(e) => return Err(e),
};
```

The function is effectively saying:

```text
"I am not deciding what to do with this error.
I pass that decision to my caller."
```

This is called **error propagation**.

---

# The `?` Operator

Code like:

```rust
let file = match File::open("hello.txt") {
    Ok(file) => file,
    Err(e) => return Err(e),
};
```

can usually be replaced with:

```rust
let file = File::open("hello.txt")?;
```

Mechanism of `?` for `Result`:

```text
Result
   ↓
   ?
 ┌─┴──────────────┐
Ok(value)        Err(error)
   ↓                 ↓
get value        return Err early
   ↓
continue the function
```

So:

```rust
operation()?
```

means approximately:

```text
if Ok
→ unwrap the value

if Err
→ return Err(...)
```

This pattern is common enough that Rust provides a dedicated operator for it. The Rust Programming Language (2)

---

## `?` and Error Conversion

There is one difference from writing:

```rust
return Err(e);
```

manually.

When `?` is used, the error can be converted into the error type returned by the current function.

This uses the `From` mechanism.

Simplified:

```text
operation error
      ↓
      ?
      ↓
From::from(...)
      ↓
current function's error type
```

This lets a function expose one final error type even when its internal operations can return different compatible error types.

`From` and traits are covered later.

---

## Multiple `?` Operators

Example from the book:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file()
    -> Result<String, io::Error>
{
    let mut username = String::new();

    File::open("hello.txt")?
        .read_to_string(&mut username)?;

    Ok(username)
}
```

Logic:

```text
open file
   ↓ Err → return error
   ↓ Ok
read file
   ↓ Err → return error
   ↓ Ok
Ok(username)
```

So `?` is well suited to chains of operations where any step can fail and the caller should decide how to respond.

---

## Do Not Reimplement What the Library Already Provides

For the common task of reading a file, the book shows an even shorter version:

```rust
use std::fs;
use std::io;

fn read_username_from_file()
    -> Result<String, io::Error>
{
    fs::read_to_string("hello.txt")
}
```

This illustrates an important general principle from the chapter: if the standard library already provides an operation with a suitable `Result`, there is no need to reproduce its internal steps manually.

---

# Where `?` Works

`?` cannot be placed in an arbitrary function.

For example:

```rust
fn main() {
    let file = File::open("hello.txt")?;
}
```

does not compile because ordinary `main` here returns:

```rust
()
```

while `?` must be able to perform an early return of a compatible error value.

The book notes that `?` is used in functions returning compatible types, in particular:

```text
Result
Option
```

---

# `?` with `Option<T>`

For:

```rust
Option<T>
```

the operator works similarly.

```text
Some(value)?
→ get value
→ continue

None?
→ return None early
```

Example from the book:

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

If:

```rust
text.lines().next()
```

returns `None`, the whole function immediately returns `None`.

If it returns:

```rust
Some(line)
```

`?` extracts `line` and execution continues.

Important:

```text
Result + ?
→ function usually returns Result

Option + ?
→ function usually returns Option
```

`?` does not automatically convert `Result` into `Option` or vice versa; the book mentions methods such as `ok` and `ok_or` for explicit conversion.

---

# `main` Can Also Return `Result`

You can write:

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("hello.txt")?;

    Ok(())
}
```

In this chapter, `Box<dyn Error>` can temporarily be understood as:

```text
"any kind of error"
```

Trait objects themselves are covered later.

If `main` returns:

```rust
Ok(())
```

the program exits successfully.

If it returns:

```rust
Err(...)
```

it exits with a nonzero status. The Rust Programming Language (2)

---

# 3. Important Details

## `panic!` or `Result`?

This is a key architectural choice.

With:

```rust
panic!()
```

the function itself decides:

```text
"recovery is impossible"
```

The caller no longer gets a chance to decide what to do.

With:

```rust
Result<T, E>
```

the decision is passed upward:

```text
function detects an error
        ↓
returns Err
        ↓
caller decides:
retry?
use another value?
show a message?
panic?
```

For this reason, the book calls `Result` a good default choice for a function that can fail. The Rust Programming Language (2)

---

## When `unwrap` / `expect` Are Acceptable

The book specifically identifies cases where panicking can be convenient:

```text
examples
prototypes
tests
```

In a prototype, this lets you write the main logic first and leave `unwrap` or `expect` as a place to replace later with proper error handling.

In a test, a panic means the test failed, so `unwrap` and `expect` are natural choices.

---

## When the Programmer Knows More Than the Compiler

Sometimes an operation returns `Result` even though the logic of a particular program guarantees that it should succeed.

Example from the book:

```rust
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP address should be valid");
```

`parse` can fail in general.

But here the string is hardcoded and known to have the expected form.

So `expect` simultaneously:

```text
unwraps Result
+
documents the program's assumption
```

If the string came from a user, that assumption would no longer hold and the error should be handled.

---

## User Error ≠ Program Error

If a failure is expected:

```text
invalid user input
invalid external data
an operation can normally fail
```

it is usually a candidate for:

```rust
Result
```

If a program contract or invariant is violated:

```text
"this state should never exist"
```

then:

```rust
panic!
```

may be justified.

The book uses out-of-bounds array access as an example where continuing with invalid memory access is unacceptable.

If a library panics when a function contract is violated, that condition should be documented in the API.

---

# Types as a Way to Reduce Errors

Rust lets you move some checks from runtime into the type system.

For example:

```rust
fn f(value: u32)
```

already guarantees that `value` is not negative.

And if a function accepts an ordinary type instead of:

```rust
Option<T>
```

it can rely on the fact that a value is present.

So instead of:

```text
repeatedly checking impossible states
```

you can try to make those states unrepresentable through an appropriate type.

---

## A User-Defined Type That Enforces an Invariant

The book demonstrates the idea with a number from `1` to `100`.

Instead of passing around:

```rust
i32
```

and repeatedly checking:

```rust
if value < 1 || value > 100
```

you can define:

```rust
struct Guess {
    value: i32,
}
```

and control construction through:

```rust
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("...");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

The field:

```rust
value
```

remains private.

External code therefore cannot construct an arbitrary invalid state while bypassing the check.

After successful construction:

```text
Guess
→ value is already validated
→ the rest of the code can rely on the invariant
```

This lets the type system concentrate validation in one place instead of repeating it across many functions.

---

# 4. What to Remember

Main Rust error map:

```text
ERROR
 │
 ├── expected / recoverable
 │       ↓
 │   Result<T, E>
 │
 └── invariant violation /
     continuation impossible
         ↓
       panic!
```

`Result`:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Remember:

```text
Ok(T)
→ successful value

Err(E)
→ error description
```

Handling:

```rust
match result {
    Ok(value) => ...,
    Err(error) => ...,
}
```

---

Shortcuts:

```text
unwrap()
→ Ok: return value
→ Err: panic

expect("reason")
→ same,
  but with your own explanation for the panic
```

For code where the caller should handle the error:

```rust
operation()?
```

read it as:

```text
Ok(value)
→ value

Err(error)
→ return Err(error)
```

---

Main signature of a function that can fail:

```rust
fn operation() -> Result<T, E>
```

For example:

```rust
fn read() -> Result<String, io::Error>
```

It tells the user immediately:

```text
"success is not guaranteed"
```

---

The `?` operator:

```text
Result + ?
→ propagate Err

Option + ?
→ propagate None
```

But:

```text
Result ↔ Option
```

are not mixed automatically.

---

Decision guide:

```text
Is the error expected?
→ Result

Can the caller decide what to do?
→ Result

Was a contract or invariant violated?
→ panic! may be appropriate

Example / prototype / test?
→ unwrap / expect may be convenient
```

And the most important architectural distinction:

```text
panic!
→ the current code decides
  to stop execution

Result
→ the current code reports a problem,
  and the caller decides what to do
```

This is one of the central ideas of error handling in Rust.

**Notes size:** approximately **9–11 thousand characters** including code.

**Time to copy into a notebook:** approximately **50–65 minutes**. If shortening, backtrace details and `panic = 'abort'` can be removed first, but keep the `panic! / Result` model, `Ok/Err`, `unwrap/expect`, propagation through `?`, and the rule for choosing between a panic and returning an error.
