# Step 11. Testing + Application Architecture

## 1. Topic

The Rust compiler checks types, ownership, borrowing, and lifetimes, but it cannot prove that a program implements exactly the business logic intended by the programmer. Automated tests are used for that. The Rust Programming Language (2)

Basic test model:

```text
prepare data
      ↓
execute code
      ↓
check the result
```

The second part of this step is ordinary application organization. Using `minigrep`, the book gradually separates the program into:

```text
main.rs
→ startup
→ input/output
→ top-level error handling

lib.rs
→ main logic
→ Config
→ run
→ functions that are convenient to test
```

This separation matters because library code can be called directly from tests without launching the complete executable every time. The Rust Programming Language (2)

---

# 2. Mechanism

## Basic Test

A Rust test is an ordinary function with the attribute:

```rust
#[test]
fn it_works() {
    let result = 2 + 2;

    assert_eq!(result, 4);
}
```

Run it with:

```bash
cargo test
```

`#[test]` tells the test runner to execute the function as a test. Cargo compiles a test binary, runs the marked functions, and reports the result. The Rust Programming Language (2)

---

## `assert!`

Checks a boolean expression:

```rust
assert!(condition);
```

The test passes when:

```text
condition == true
```

For example:

```rust
assert!(larger.can_hold(&smaller));
```

Opposite check:

```rust
assert!(!smaller.can_hold(&larger));
```

If the expression is false, the test panics and is considered failed.

---

## `assert_eq!` and `assert_ne!`

For comparing values, use:

```rust
assert_eq!(actual, expected);
```

and:

```rust
assert_ne!(a, b);
```

On failure, `assert_eq!` and `assert_ne!` print the compared values, so they usually provide more information than:

```rust
assert!(a == b);
```

For these macros to work, compared types must implement `PartialEq`, and for diagnostic output they must implement `Debug`. For your own types, these can often be derived with:

```rust
#[derive(PartialEq, Debug)]
```

The Rust Programming Language (2)

---

## Failure Message

Assertions can include a message:

```rust
assert!(
    result.contains(name),
    "Greeting did not contain name: {result}"
);
```

The same applies to:

```text
assert!
assert_eq!
assert_ne!
```

Additional arguments are formatted similarly to `format!`.

Practical effect:

```text
assertion failed
+
context explaining the reason
→ faster diagnosis of what broke
```

---

# Testing `panic!`

Sometimes the correct behavior is for a function to panic when a contract is violated.

For that:

```rust
#[test]
#[should_panic]
fn greater_than_100() {
    Guess::new(200);
}
```

The test passes if the code actually panics.

But plain `should_panic` is too broad:

```text
any panic occurs
→ test passes
```

Even if the panic happened for the wrong reason.

So a more precise form is:

```rust
#[test]
#[should_panic(expected = "less than or equal to 100")]
fn greater_than_100() {
    Guess::new(200);
}
```

Now the panic message must contain the specified substring. The Rust Programming Language (2)

---

# Tests Returning `Result`

A test does not have to finish by panicking.

It can return:

```rust
Result<(), String>
```

Conceptually:

```rust
#[test]
fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
```

Model:

```text
Ok(())
→ test passed

Err(...)
→ test failed
```

This is useful when the test already calls operations returning `Result` and you want to use `?`.

The book specifically notes that `#[should_panic]` is not used with this style: when testing a `Result` error, work with the `Result` itself. The Rust Programming Language (2)

---

# Organizing Unit Tests

Typical structure:

```rust
pub fn add_two(a: u64) -> u64 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(2), 4);
    }
}
```

Here:

```text
#[cfg(test)]
→ module is compiled only during cargo test

mod tests
→ separate test module

use super::*
→ gives tests access to the parent module's contents
```

Unit tests are usually placed next to the code they test in `src`. They can test private functions because they are a child module of the tested module. The Rust Programming Language (2)

---

# Integration Tests

Integration tests live separately:

```text
project/
├── src/
│   └── lib.rs
└── tests/
    └── integration_test.rs
```

For example:

```rust
use adder::add_two;

#[test]
fn it_adds_two() {
    assert_eq!(add_two(2), 4);
}
```

Each file in `tests/` is compiled as a separate crate.

That means an integration test sees the library **as an external user**:

```text
can use public API
cannot directly use private API
```

Files under `tests/` do not need an explicit:

```rust
#[cfg(test)]
```

because Cargo already knows they are integration tests. The Rust Programming Language (2)

---

## Unit vs Integration

```text
UNIT TEST
→ next to the code
→ small unit
→ can test private API

INTEGRATION TEST
→ tests/ directory
→ external crate
→ public API only
→ checks that parts work together
```

Both are needed for different levels of verification. The Rust Programming Language (2)

---

# Useful Test Commands

Main command:

```bash
cargo test
```

One test:

```bash
cargo test one_hundred
```

Or filter by part of a name:

```bash
cargo test add
```

Tests may run in parallel by default, so they should not unnecessarily depend on shared mutable state. The Rust Programming Language (2)

An expensive test can be marked:

```rust
#[test]
#[ignore]
fn expensive_test() {
    ...
}
```

Run only ignored tests:

```bash
cargo test -- --ignored
```

This is secondary functionality, but useful to recognize in a real project. The Rust Programming Language (2)

---

# Application Architecture

## The Problem with a Large `main`

At the start of `minigrep`, everything could be written inside:

```rust
fn main() {
    ...
}
```

But then `main` becomes responsible for all of these at once:

```text
reading CLI arguments
parsing arguments
configuration
reading a file
searching
output
error handling
```

Such code is difficult to test and extend independently.

So the book gradually separates responsibilities.

---

# CLI Arguments

Program arguments can be obtained through:

```rust
use std::env;

let args: Vec<String> = env::args().collect();
```

When running:

```bash
cargo run -- needle haystack
```

the vector conceptually contains:

```text
args[0] → name/path of the executed program
args[1] → "needle"
args[2] → "haystack"
```

Here `--` separates Cargo arguments from arguments for the program itself. The Rust Programming Language (2)

---

# `Config` Instead of Separate Variables

Instead of storing related parameters independently:

```rust
query
file_path
ignore_case
```

the book groups them into a structure:

```rust
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
```

Reason:

```text
these values together describe
the configuration of one run
```

The structure makes that relationship explicit.

An earlier tuple-based version was replaced by `Config` because named fields express the meaning of data more clearly and are easier to maintain. The Rust Programming Language (2)

---

# `Config::build`

Argument parsing is moved out of `main`:

```rust
impl Config {
    fn build(args: &[String])
        -> Result<Config, &'static str>
    {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {
            query,
            file_path,
            ignore_case: false,
        })
    }
}
```

The architectural idea matters more than the exact implementation:

```text
main
should not know details such as:
"which index means query?"

Config::build
takes raw input
and turns it into meaningful configuration
```

Also, `build` returns `Result` instead of calling `panic!` because missing arguments are an expected user-input error. The Rust Programming Language (2)

---

# `main` Handles Configuration Errors

Approximate model:

```rust
let config = Config::build(&args)
    .unwrap_or_else(|err| {
        eprintln!(
            "Problem parsing arguments: {err}"
        );

        process::exit(1);
    });
```

Responsibility is split:

```text
Config::build
→ detects the problem
→ returns Err

main
→ decides how to show the error to the user
→ terminates the process
```

So library logic reports an error, while the top level determines user-facing behavior.

---

# `run` — Main Application Execution

After creating `Config`, the program's actual work is moved into:

```rust
fn run(config: Config)
    -> Result<(), Box<dyn Error>>
{
    let contents =
        fs::read_to_string(config.file_path)?;

    // main work

    Ok(())
}
```

The signature:

```text
Result<(), Box<dyn Error>>
```

means:

```text
success
→ Ok(())
→ no useful return value

failure
→ Err(...)
```

`()` indicates that the function is executed for its effects rather than to produce a useful value.

`?` propagates a file-reading error to the caller. The Rust Programming Language (2)

---

# Reading a File

Main operation:

```rust
let contents =
    fs::read_to_string(file_path)?;
```

`fs::read_to_string`:

```text
accepts a path
→ tries to open the file
→ reads its contents
→ returns Result<String, ...>
```

So file I/O naturally fits into the already studied `Result` + `?` model. The Rust Programming Language (2)

---

# Splitting `main.rs` and `lib.rs`

After refactoring, the structure becomes roughly:

```text
src/
├── main.rs
└── lib.rs
```

`main.rs`:

```text
obtain external input
        ↓
create Config
        ↓
call run
        ↓
print error
        ↓
set exit status
```

`lib.rs`:

```text
Config
run
search
search_case_insensitive
main logic
```

As a result, most functionality becomes a library API that can be tested directly. The Rust Programming Language (2)

This is one of the chapter's key architectural ideas.

---

# Environment Variables

Configuration can come from more than the CLI.

For example, the book uses:

```text
IGNORE_CASE
```

and checks it through `std::env`.

Model:

```text
CLI arguments
        ┐
        ├──→ Config
environment variables
        ┘
```

`Config` then contains:

```rust
ignore_case: bool
```

and `run` selects the required behavior:

```text
ignore_case == false
→ search(...)

ignore_case == true
→ search_case_insensitive(...)
```

The book also notes that if one setting can come from several sources, the program needs to define which source has priority. The Rust Programming Language (2)

---

# `stdout` and `stderr`

Program results and errors should be separated.

Normal output:

```rust
println!("...");
```

goes to:

```text
stdout
```

Errors:

```rust
eprintln!("...");
```

go to:

```text
stderr
```

Why does this matter?

For example:

```bash
cargo run -- to poem.txt > output.txt
```

redirects `stdout` into a file.

If error messages are also printed with `println!`, they end up in `output.txt` together with normal results.

Therefore:

```text
stdout
→ successful output data

stderr
→ diagnostics and errors
```

The book changes error output in `main` to `eprintln!` so redirecting normal output does not swallow errors. The Rust Programming Language (2)

---

# 3. Important Details

## Tests and Architecture Are Directly Connected

If all logic lives in:

```text
src/main.rs
```

integration tests cannot easily use it directly.

A binary crate is intended to be run, while a library crate can provide functions to other crates.

So the book recommends:

```text
main.rs
→ thin wrapper

lib.rs
→ real logic
```

This improves all of the following:

```text
testability
modularity
error handling
reuse
```

The Rust Programming Language (2)

---

## `main` Should Be Small

Good final model for `main`:

```text
1. obtain input
2. build Config
3. call run
4. handle top-level error
```

Not:

```text
main
→ contains all application business logic
```

---

## Do Not Use `panic!` for Ordinary Bad CLI Input

For example:

```text
user forgot an argument
```

is an expected situation.

Therefore:

```text
Config::build
→ Result
```

is better than unconditional:

```rust
panic!("not enough arguments");
```

The user gets a normal message instead of a technical panic/backtrace.

---

## Do Not Ignore `Result`

If:

```rust
run(config);
```

returns `Result` and it is not handled, Rust issues a warning.

You must:

```text
handle Err
or
explicitly propagate it
```

In `minigrep`, `main` handles the error returned by `run`. The Rust Programming Language (2)

---

## TDD as a Working Cycle

Chapter 12 demonstrates a simple test-driven development cycle:

```text
1. write a test
2. confirm that it fails
3. write the minimum code to pass
4. refactor
5. repeat
```

For example, a test for `search` is written first, then a minimal empty implementation, then the working search logic.

The book does not present TDD as the only development approach, but uses it as a way to grow `minigrep` gradually. The Rust Programming Language (2)

---

# 4. What to Remember

Minimal test structure:

```rust
#[test]
fn something_works() {
    let result = operation();

    assert_eq!(result, expected);
}
```

Main assertions:

```text
assert!(condition)
→ condition must be true

assert_eq!(a, b)
→ a == b

assert_ne!(a, b)
→ a != b
```

Testing panic:

```rust
#[test]
#[should_panic(expected = "text")]
fn invalid_value_panics() {
    ...
}
```

---

Unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        ...
    }
}
```

```text
→ live next to the code
→ can test private parts
```

Integration tests:

```text
tests/
└── integration_test.rs
```

```text
→ separate crate
→ uses the library from the outside
→ sees only public API
```

---

Main application architecture:

```text
main.rs
│
├─ obtain args
├─ create Config
├─ call run
├─ print errors
└─ terminate process

lib.rs
│
├─ Config
├─ Config::build
├─ run
├─ search
└─ main logic
```

---

CLI:

```rust
env::args()
```

File:

```rust
fs::read_to_string(path)?
```

Environment variables:

```text
std::env
→ additional Config source
```

Output:

```text
println!
→ stdout
→ normal result

eprintln!
→ stderr
→ errors
```

---

Application error handling:

```text
Config::build(...)
       ↓
Result<Config, ...>
       ↓
main

run(config)
       ↓
Result<(), ...>
       ↓
main
```

That is:

```text
lower level
→ returns an error

upper level
→ decides how to present it to the user
```

---

The most important connection in this step:

```text
WELL-SEPARATED CODE
        ↓
easier to test
        ↓
errors are easier to handle
        ↓
main becomes small
        ↓
main logic lives in a library crate
```

Final model:

```text
compiler checks
→ type and memory safety

tests check
→ expected program behavior

architecture
→ makes that behavior
  convenient to test
```

Testing and architecture are therefore studied together: **the less logic is hidden inside `main`, the easier it is to call directly, verify its results, and handle input/output and errors separately.**

**Notes size:** approximately **10–12 thousand characters** including code.

**Time to copy into a notebook:** approximately **55–70 minutes**. If shortening, `#[ignore]` and some test-filter commands can be removed first; keep `#[test]`, assertions, `should_panic`, unit/integration tests, and the `main.rs → Config → run → lib.rs` model.
