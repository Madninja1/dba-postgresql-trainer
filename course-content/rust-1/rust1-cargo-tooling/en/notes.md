# Step 13. Cargo as a Working Tool

## 1. Topic

Up to this point, Cargo has mostly been used for:

```text
cargo new
cargo check
cargo build
cargo run
cargo test
```

Chapter 14 presents Cargo as a full Rust project-management tool:

```text
Cargo
├─ configures compilation modes
├─ generates documentation
├─ publishes crates
├─ manages multiple packages through workspaces
├─ installs ready-made CLI tools
└─ supports custom subcommands
```

The main topics are release profiles, publishing to crates.io, workspaces, `cargo install`, and extending Cargo. The Rust Programming Language (2)

---

# 2. Mechanism

## Build Profiles

Cargo has different **compilation profiles**.

The two main profiles are:

```text
dev
→ cargo build
→ development

release
→ cargo build --release
→ release build
```

They use different compiler settings.

Typical tradeoff:

```text
dev
→ faster compilation
→ fewer optimizations

release
→ slower compilation
→ more optimizations
→ faster resulting program
```

---

## Configuring Profiles

Settings are specified in `Cargo.toml`:

```toml
[profile.dev]
opt-level = 1
```

If the section is absent, Cargo uses default values.

In the book, the defaults for `opt-level` are:

```text
dev     → 0
release → 3
```

`opt-level` controls the compiler's optimization level in the `0..3` range. Higher optimization generally requires more compilation time. The Rust Programming Language (2)

Main model:

```text
frequent development
→ faster compilation matters more

finished release
→ program performance matters more
```

---

# Library Documentation

## `///` — Documentation for an Item

Ordinary comment:

```rust
// comment
```

Documentation comment:

```rust
/// Adds one to the number.
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

`///` documents the item that follows it:

```text
/// ...
↓
function / struct / enum / ...
```

Documentation supports Markdown. The Rust Programming Language (2)

---

## Examples in Documentation

You can write:

```rust
/// Adds one.
///
/// # Examples
///
/// ```
/// let result = my_crate::add_one(5);
/// assert_eq!(result, 6);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

Generate documentation with:

```bash
cargo doc
```

The result is placed in:

```text
target/doc
```

Convenient form:

```bash
cargo doc --open
```

→ generate documentation and open it in a browser. The Rust Programming Language (2)

---

## Documentation Examples Are Tests

Code inside documentation comments is checked by:

```bash
cargo test
```

So:

```text
documentation
    ↓
code example
    ↓
doc-test
    ↓
Cargo checks
that the example still works
```

This protects against:

```text
API changed
+
example stayed outdated
```

The Rust Programming Language (2)

---

## Useful Documentation Sections

The book specifically highlights:

```text
# Examples
→ usage example

# Panics
→ when the function may panic

# Errors
→ which Err values are possible and why

# Safety
→ requirements of an unsafe function
```

Not every function needs all of these sections; they are guidelines for documenting important API properties. The Rust Programming Language (2)

---

## `//!` — Documentation for a Container

`///` documents the **next item**.

`//!` documents the **item that contains the comment**.

For example, at the beginning of `src/lib.rs`:

```rust
//! # My Crate
//!
//! Utilities for ...
```

this description applies to the whole crate.

The same form can be used inside a module for its general documentation. The Rust Programming Language (2)

Remember:

```text
/// item
→ documentation for the next item

//! module/crate
→ documentation for the current container
```

---

# Public API and `pub use`

A library's internal structure does not have to match what users see.

For example, internally it may be convenient to have:

```text
crate
├─ kinds
│  └─ PrimaryColor
└─ utils
   └─ mix
```

Without re-exporting, a user writes:

```rust
use art::kinds::PrimaryColor;
use art::utils::mix;
```

A simpler API can be exposed:

```rust
pub use self::kinds::PrimaryColor;
pub use self::utils::mix;
```

Now:

```rust
use art::PrimaryColor;
use art::mix;
```

The Rust Programming Language (2)

Main idea:

```text
internal architecture
≠ necessarily public API
```

`pub use` lets you change how the library is presented to users without completely restructuring internal modules.

---

# Publishing a Crate

The book publishes with:

```bash
cargo publish
```

Before publishing, `Cargo.toml` must contain the required metadata.

Example:

```toml
[package]
name = "my_crate"
version = "0.1.0"
edition = "2021"
description = "Description of the crate."
license = "MIT"
```

The name on crates.io must be unique. The book also requires a description and a license for publishing; `license-file` can be used instead of a license identifier when the license text is stored in a separate file. The Rust Programming Language (2)

---

## Versions After Publishing

A published version is not overwritten.

To release a new one:

```text
1. change version in Cargo.toml
2. choose a number according to Semantic Versioning
3. run cargo publish again
```

The book emphasizes that publishing a version is permanent: already uploaded contents cannot simply be replaced. The Rust Programming Language (2)

---

## `cargo yank`

If a published version turns out to be problematic:

```bash
cargo yank --vers 1.0.1
```

This **does not delete** the version.

Mechanism:

```text
existing projects
with this version in Cargo.lock
→ continue to work

new dependency resolutions
→ no longer select this version
```

Undo:

```bash
cargo yank --vers 1.0.1 --undo
```

Important:

```text
yank ≠ delete
```

If a secret was accidentally published, `yank` does not remove it. The book says the compromised secret should be replaced immediately. The Rust Programming Language (2)

---

# Workspaces

## What Is a Workspace?

A **workspace** combines several related Cargo packages that are developed together.

Main shared resources:

```text
one Cargo.lock
+
one shared target/
```

The Rust Programming Language (2)

Example:

```text
add/
├─ Cargo.toml
├─ Cargo.lock
├─ target/
├─ adder/
│  ├─ Cargo.toml
│  └─ src/main.rs
└─ add_one/
   ├─ Cargo.toml
   └─ src/lib.rs
```

The root `Cargo.toml` describes the workspace:

```toml
[workspace]
resolver = "3"
members = [
    "adder",
    "add_one",
]
```

Such a root manifest does not contain an ordinary `[package]` section; it describes the workspace itself.

---

## Why Share `target`

Even if one package inside the workspace is built separately:

```bash
cargo build
```

artifacts are placed in the shared:

```text
workspace/target/
```

This lets several packages reuse compilation results and reduces unnecessary recompilation. The Rust Programming Language (2)

---

## Dependencies Between Workspace Members

Putting two packages in the same workspace **does not automatically make them dependencies of one another**.

If `adder` uses `add_one`, that dependency must be declared explicitly:

```toml
[dependencies]
add_one = { path = "../add_one" }
```

Then:

```text
adder
  ↓ depends on
add_one
```

The Rust Programming Language (2)

---

## External Dependencies in a Workspace

A shared `Cargo.lock` does not mean an external dependency is automatically available to every package.

If `add_one` uses `rand`:

```toml
[dependencies]
rand = "..."
```

that does not allow writing:

```rust
use rand;
```

inside `adder` immediately.

`adder/Cargo.toml` must also declare `rand` as its dependency.

Summary:

```text
Cargo.lock
→ shared version selection

Cargo.toml of each package
→ its own list of allowed dependencies
```

The Rust Programming Language (2)

---

## Workspace Commands

Build from the root:

```bash
cargo build
```

Work with a specific package:

```bash
cargo run -p adder
```

Test the whole workspace:

```bash
cargo test
```

Test one package:

```bash
cargo test -p add_one
```

If crates from a workspace are published, each is published separately. The Rust Programming Language (2)

---

# `cargo install`

Cargo can install executable Rust packages:

```bash
cargo install package_name
```

For example, the book shows:

```bash
cargo install ripgrep
```

This installs packages containing a **binary target** — a program that can be run directly. `cargo install` is not used to install ordinary library dependencies for a project. The Rust Programming Language (2)

---

## Where Programs Are Installed

With a typical Rust installation through `rustup`, binaries go to:

```text
$HOME/.cargo/bin
```

If that directory is in `$PATH`, the installed program can be launched by name:

```text
cargo install ...
        ↓
~/.cargo/bin/tool
        ↓
tool
```

---

# Custom Cargo Commands

Cargo can be extended without modifying Cargo itself.

If `$PATH` contains a program named:

```text
cargo-something
```

it can be run as:

```bash
cargo something
```

For example:

```text
cargo-abc
   ↓
cargo abc
```

Custom subcommands can be listed together with other commands using:

```bash
cargo --list
```

This lets an external tool look like a natural Cargo extension. The Rust Programming Language (2)

---

# 3. Important Details

Here Cargo operates at several levels:

```text
COMPILATION
→ profiles

LIBRARY API
→ cargo doc
→ ///, //!
→ pub use

DISTRIBUTION
→ cargo publish
→ cargo yank

LARGE PROJECT
→ workspace

TOOLS
→ cargo install
→ custom subcommands
```

Most important distinctions:

```text
cargo build
→ dev profile

cargo build --release
→ release profile
```

and:

```text
Cargo.toml package
→ describes one package

Cargo.toml workspace
→ organizes several packages
```

---

Do not confuse workspace membership with dependency relationships:

```text
same workspace
≠ another crate is automatically available through use
```

The relationship still has to be declared:

```toml
[dependencies]
some_crate = { path = "../some_crate" }
```

---

Do not confuse a shared `Cargo.lock` with shared dependencies:

```text
Cargo.lock
→ locks jointly selected versions

[dependencies]
→ determines what a particular package may use
```

---

Documentation is part of the workflow:

```text
/// API docs
      ↓
cargo doc
      ↓
HTML

code examples
      ↓
cargo test
      ↓
doc-tests
```

So an example in documentation both explains the API and is checked for freshness.

---

# 4. What to Remember

Profiles:

```bash
cargo build
```

→ `dev`.

```bash
cargo build --release
```

→ `release`.

Configuration:

```toml
[profile.dev]
opt-level = 1
```

---

Documentation:

```rust
/// Documentation for next item.

//! Documentation for current module/crate.
```

Commands:

```bash
cargo doc
cargo doc --open
cargo test
```

Code examples in documentation participate in doc-tests.

---

Convenient public API:

```rust
pub use crate::internal::SomeType;
```

Remember:

```text
pub use
→ re-export
→ internal modules can be hidden
  behind a more convenient public API
```

---

Publishing:

```bash
cargo publish
```

New version:

```text
change version
→ cargo publish
```

Problematic version:

```bash
cargo yank --vers 1.0.1
```

Restore it:

```bash
cargo yank --vers 1.0.1 --undo
```

```text
yank
≠ deleting a version
```

---

Workspace:

```toml
[workspace]
members = [
    "app",
    "library",
]
```

Main model:

```text
WORKSPACE
├─ shared Cargo.lock
├─ shared target/
├─ package A
├─ package B
└─ package C
```

But:

```text
A uses B
→ the dependency must be declared explicitly
```

For example:

```toml
[dependencies]
library = { path = "../library" }
```

---

Main workspace commands:

```bash
cargo build
cargo test

cargo run -p package
cargo test -p package
```

---

Install a CLI tool:

```bash
cargo install crate_name
```

Typical location:

```text
$HOME/.cargo/bin
```

Extend Cargo:

```text
cargo-tool
→ cargo tool
```

List available commands:

```bash
cargo --list
```

---

Final model:

```text
DEVELOP
→ cargo build
→ dev profile

RELEASE
→ cargo build --release
→ release profile

DOCUMENT
→ cargo doc
→ doc-tests

ORGANIZE
→ workspace

PUBLISH
→ cargo publish
→ cargo yank

INSTALL TOOLS
→ cargo install

EXTEND
→ cargo-something
→ cargo something
```

Main idea: **Cargo is not just a build command. It ties together the Rust project lifecycle: development, compiler configuration, documentation, testing, multi-package organization, publishing, and tool installation.**

**Notes size:** approximately **8–10 thousand characters** including code.

**Time to copy into a notebook:** approximately **40–55 minutes**. For long-term memory, focus especially on profiles, `cargo doc`, `pub use`, the workspace model, `cargo publish/yank`, and `cargo install`.
