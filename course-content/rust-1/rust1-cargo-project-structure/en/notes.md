# Step 1. Rust + Cargo + Project Structure

## 1. Topic

Rust is a compiled language: source code is first compiled into an executable file, after which that file can be run independently of the Rust compiler. For a single file you can use `rustc`, but real projects are usually managed through **Cargo** — Rust's build system and dependency manager. The Rust Programming Language The Rust Programming Language

Main tools:

```text
rustup   — install and manage Rust versions
rustc    — Rust compiler
cargo    — create, build, and manage projects and dependencies
rustfmt  — format Rust code
```

`rustup` also installs local documentation; you can open it with:

```bash
rustup doc
```

Update Rust:

```bash
rustup update
```

The Rust Programming Language

---

# 2. Mechanism

## Compilation

A minimal Rust program has a `main` function:

```rust
fn main() {
    println!("Hello, world!");
}
```

`main` is the entry point of an executable program. `fn` declares a function, and its body is enclosed in `{}`. `println!` is a **macro**, not an ordinary function: `!` indicates a macro invocation. The Rust Programming Language The Rust Programming Language

A single file can be compiled directly:

```bash
rustc main.rs
```

After successful compilation, an executable binary is produced. Compilation and execution are separate stages. The Rust Programming Language

For real development, Cargo is usually used instead of invoking `rustc` directly.

---

## Cargo

Cargo performs several tasks:

- creates the project structure;
- compiles code;
- manages external dependencies;
- downloads and compiles dependencies;
- reuses already built parts of the project when they have not changed. The Rust Programming Language The Rust Programming Language

Create a project:

```bash
cargo new my_project
cd my_project
```

Cargo creates roughly this structure:

```text
my_project/
├── Cargo.toml
└── src/
    └── main.rs
```

Project source code is stored in `src`. Project configuration is stored in `Cargo.toml`. Cargo can also initialize a Git repository. The Rust Programming Language The Rust Programming Language

---

## Cargo.toml

`Cargo.toml` is Cargo's main configuration file, written in TOML format.

Example:

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
```

The `[package]` section contains parameters of the package itself: name, version, and Rust edition.

The `[dependencies]` section contains the project's external dependencies. The Rust Programming Language

For example:

```toml
[dependencies]
rand = "0.8.5"
```

When Cargo builds the project:

```text
Cargo.toml
    ↓
determine dependencies
    ↓
download required crates
    ↓
compile dependencies
    ↓
compile the project
```

Dependencies are downloaded from a registry; the book identifies **Crates.io** as the place where public Rust packages are published. Transitive dependencies are downloaded automatically as well. The Rust Programming Language

---

## Cargo.lock and Reproducible Builds

`Cargo.toml` specifies **allowed versions**, while `Cargo.lock` records the exact versions actually selected by Cargo.

On the first build, Cargo determines suitable dependency versions and writes them to `Cargo.lock`. Subsequent builds use the locked versions, which allows the same project to be rebuilt with the same dependency set. The Rust Programming Language

Simplified:

```text
Cargo.toml
"which versions are allowed"

        ↓ Cargo selects versions

Cargo.lock
"which exact versions are used"
```

`Cargo.lock` should not be edited manually: Cargo manages it. The Rust Programming Language

Update dependencies within the constraints of `Cargo.toml`:

```bash
cargo update
```

The command resolves versions again and updates `Cargo.lock`. The Rust Programming Language

---

## Dependency Versions

Cargo uses semantic versioning.

In the example from the book:

```toml
rand = "0.8.5"
```

is equivalent to the requirement:

```text
^0.8.5
```

That means compatible versions at least `0.8.5` but below `0.9.0` are allowed. Moving to the next incompatible series requires changing the requirement in `Cargo.toml`. The Rust Programming Language

---

# 3. Important Details

## Main Cargo Commands

For everyday work, it is enough to remember a few commands.

### Create a Project

```bash
cargo new project_name
```

### Check the Code

```bash
cargo check
```

Checks whether the project compiles but does not produce an executable file. It is therefore usually faster than a full build. The Rust Programming Language

### Build

```bash
cargo build
```

A normal build is created in:

```text
target/debug/
```

The Rust Programming Language

### Build and Run Immediately

```bash
cargo run
```

If the sources have not changed, Cargo can reuse the already built binary instead of compiling it again. The Rust Programming Language

### Release Build

```bash
cargo build --release
```

Result:

```text
target/release/
```

A release build enables optimizations. It takes longer to compile but is intended for fast final execution. The book recommends measuring performance using a release build. The Rust Programming Language

Summary:

```text
cargo check
    ↓
quickly check the code

cargo run
    ↓
build + run

cargo build
    ↓
produce a debug binary

cargo build --release
    ↓
produce an optimized binary
```

---

## `use` and Scope

Some standard-library items are automatically available to every Rust program. This set is called the **prelude**.

If the required item is not in the prelude, it can be brought into scope explicitly:

```rust
use std::io;
```

After that:

```rust
io::stdin()
```

Without `use`, you would need to write the full path:

```rust
std::io::stdin()
```

The Rust Programming Language The Rust Programming Language

For now it is enough to understand:

```text
std          standard library
std::io      io module inside std
::           move along a path to an item
use          bring a path/item into the current scope
```

The module system will be covered in detail later.

---

## Function, Method, and Associated Function

Three forms of calls appear very early in the book.

### Ordinary Function

```rust
some_function()
```

### Associated Function

```rust
String::new()
```

`new` is associated with the `String` type, but it is not called on a particular object. `::` is used to access an item associated with a type.

`String::new()` creates a new empty `String` value. The Rust Programming Language

### Method

```rust
object.method()
```

For example:

```rust
io::stdin().read_line(...)
```

A method is called on a particular value through `.`. The Rust Programming Language

For now, only the syntax difference matters:

```text
Type::function()   associated function

value.method()     method on a value
```

---

## Variables: the First Rule

A variable is created with:

```rust
let x = 5;
```

Rust variables are **immutable by default**.

To change a value, mutability must be explicit:

```rust
let mut x = 5;
```

The Rust Programming Language

This is only the basic rule for now; variables and `mut` are covered in detail in the next step.

---

## References: First Introduction Only

The book already contains:

```rust
&mut guess
```

`&` denotes a reference: code can access data without having to pass a separate copy.

References, like variables, are immutable by default:

```rust
&value
```

Mutable reference:

```rust
&mut value
```

You do not need to memorize the detailed borrowing rules yet — they are a separate major mechanism in the ownership chapter. The Rust Programming Language

---

## `Result`: First Introduction

Some operations can fail, so Rust returns `Result`.

It has two variants:

```text
Ok    operation succeeded
Err   an error occurred
```

`Ok` contains the successful value; `Err` contains information about the error. The Rust Programming Language

Example from the book:

```rust
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

`expect`:

```text
Ok(value)  → returns value
Err(...)   → terminates the program with the specified message
```

Rust warns when a returned `Result` is simply ignored because a possible error has been left unhandled. The Rust Programming Language

Full error handling will be covered in a separate step.

---

# What to Remember

The main model of the first step:

```text
Rust code
   ↓
Cargo
   ↓
rustc + dependencies
   ↓
executable file
```

And separately:

```text
Cargo.toml
    ↓
what the project requires

Cargo.lock
    ↓
what was actually selected

src/
    ↓
source code

target/
    ↓
build results
```

Minimal working command set:

```bash
cargo new project
cargo check
cargo run
cargo build
cargo build --release
cargo update
```

You need to distinguish clearly:

```text
rustc                 compiler
Cargo                 project and build management

cargo check           check
cargo run             build + run
cargo build           debug build
cargo build --release release build

Cargo.toml            project requirements
Cargo.lock            locked dependency versions

use std::io            bring an item into scope
String::new()          associated function
value.method()         method
println!               macro
```

And a few rules that already show the Rust style:

```rust
let x = 5;       // immutable
let mut x = 5;   // mutable

&x               // reference
&mut x           // mutable reference
```

But **ownership, borrowing, `Result`, modules, and types are only an introduction here**. It is better to study them fully in their corresponding steps rather than duplicate material.

---

**Notes size:** approximately **7,500–8,500 characters** including code.

**Estimated time to copy into a notebook:** **40–55 minutes**. If you write diagrams and code very compactly — about **35–45 minutes**.
