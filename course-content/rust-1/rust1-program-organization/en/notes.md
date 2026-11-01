# Step 6. Program Organization

## 1. Topic

Rust's module system solves four tasks:

```text
package → combines one or more crates
crate   → compilation unit
module  → organizes code inside a crate
path    → identifies where an item is located
```

In addition:

```text
pub     → controls visibility
use     → brings a path into the current scope
```

The book describes program organization specifically in terms of packages, crates, modules, and paths. The Rust Programming Language (2)

Main hierarchy:

```text
PACKAGE
│
├── CRATE
│    │
│    └── MODULE TREE
│         ├── module
│         │    ├── function
│         │    ├── struct
│         │    └── submodule
│         └── module
│
└── CRATE
```

---

# 2. Mechanism

## Package and Crate

### Crate

A **crate** is the smallest amount of code that the Rust compiler considers at one time.

Even if you run:

```bash
rustc main.rs
```

the single file passed to the compiler is treated as a crate.

There are two kinds:

```text
binary crate
library crate
```

### Binary Crate

Compiles to an executable file.

It must have:

```rust
fn main()
```

Examples from the book include a command-line program and a server.

### Library Crate

Does not contain `main` and does not produce a standalone executable.

Its purpose is to provide functionality to other code. The Rust Programming Language (2)

---

## Package

A **package** is one or more crates that provide a set of functionality and are described by:

```text
Cargo.toml
```

Package constraints:

```text
may contain:
→ any number of binary crates

may contain:
→ at most one library crate

must contain:
→ at least one crate
```

The Rust Programming Language (2)

---

## How Cargo Determines Crates

If there is:

```text
src/main.rs
```

Cargo treats it as the root of a binary crate with the package name.

If there is:

```text
src/lib.rs
```

it is the root of a library crate with the same name.

If both are present:

```text
src/
├── main.rs
└── lib.rs
```

the package contains:

```text
1 binary crate
+
1 library crate
```

Additional binary crates can be placed in:

```text
src/bin/
```

where each file corresponds to a separate binary crate. The Rust Programming Language (2)

---

# Modules

## `mod`

Modules group related items inside a crate.

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }

    mod serving {
        fn take_order() {}
    }
}
```

A module can contain:

```text
functions
structs
enums
constants
traits
other modules
```

Its main job is to organize code while also controlling visibility. The Rust Programming Language (2)

---

## Module Tree

The previous code creates this tree:

```text
crate
└── front_of_house
    ├── hosting
    │   └── add_to_waitlist
    └── serving
        └── take_order
```

The root is the implicit module:

```text
crate
```

`src/main.rs` and `src/lib.rs` are the root modules of their respective crates. The Rust Programming Language (2)

Relationships between modules:

```text
parent
│
├── child
└── child

two children of one parent
→ siblings
```

---

# Paths

A **path** is used to refer to an item in the module tree.

Path components are separated by:

```rust
::
```

There are two kinds of paths.

---

## Absolute Path

Starts from the crate root.

For the current crate:

```rust
crate::front_of_house::hosting::add_to_waitlist()
```

That is:

```text
crate
  ↓
front_of_house
  ↓
hosting
  ↓
add_to_waitlist
```

For an item in an external crate, an absolute path starts with that crate's name. The Rust Programming Language (2)

---

## Relative Path

Starts relative to the current module.

For example:

```rust
front_of_house::hosting::add_to_waitlist()
```

You can also use:

```text
self
super
```

The book notes that the choice between absolute and relative paths depends on how code might move during reorganization. In the example, the authors prefer absolute paths because definitions and the code that uses them can be moved more independently. The Rust Programming Language (2)

---

## `super`

`super` means moving to the parent module.

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        super::deliver_order();
    }
}
```

Diagram:

```text
crate
├── deliver_order
│
└── back_of_house
     └── fix_incorrect_order
             │
             └─ super::deliver_order
```

Conceptually, `super` resembles:

```text
..
```

in a filesystem path. The Rust Programming Language (2)

---

# 3. Important Details

## Private by Default

In Rust, items are private to their parent modules by default.

This applies to:

```text
modules
functions
structs
enums
methods
constants
```

For example:

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}
```

`hosting` is not accessible from the parent code.

Main idea:

```text
private by default
        ↓
implementation details are hidden
        ↓
only the required API is exposed
```

Child modules can still access items in their ancestor modules. The Rust Programming Language (2)

---

## `pub`

To make an item accessible:

```rust
pub
```

For example:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
```

Important: making only the module public is not enough.

```rust
pub mod hosting {
    fn add_to_waitlist() {}
}
```

Here:

```text
hosting
→ public

add_to_waitlist
→ still private
```

`pub` on a container does not automatically make its contents public. The Rust Programming Language (2)

---

# `pub struct` and Fields

For a structure:

```rust
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}
```

the structure itself is accessible externally, but its fields still follow privacy rules independently.

Here:

```text
Breakfast       public
toast           public
seasonal_fruit  private
```

If a structure contains private fields, external code cannot fill them directly when constructing an instance. In the book's example, the structure provides a public associated function for this purpose. The Rust Programming Language (2)

This makes it possible to hide internal state behind a public API.

---

# `pub enum`

The rule is different for enums:

```rust
pub enum Appetizer {
    Soup,
    Salad,
}
```

If the `enum` itself is public, its variants are automatically public:

```text
Appetizer::Soup
Appetizer::Salad
```

So:

```text
pub struct
→ fields are still private by default

pub enum
→ variants are accessible
```

The Rust Programming Language (2)

---

# `use`

Repeating long paths is inconvenient:

```rust
crate::front_of_house::hosting::add_to_waitlist();
```

`use` brings a path into scope:

```rust
use crate::front_of_house::hosting;

hosting::add_to_waitlist();
```

`use` does not change where the item lives in the module tree — it creates a convenient name in the current scope. The Rust Programming Language (2)

---

## `use` Applies Only in Its Scope

```rust
use crate::front_of_house::hosting;
```

The name `hosting` is available only in the scope containing this `use`.

Entering a child module does not automatically make that imported name available there. The Rust Programming Language (2)

So:

```text
use
↓
local import of a name
↓
not global for the entire crate
```

---

# Idiomatic `use`

The book distinguishes the common import style for functions and types.

For a function, its parent module is usually imported:

```rust
use crate::front_of_house::hosting;

hosting::add_to_waitlist();
```

rather than:

```rust
use crate::front_of_house::hosting::add_to_waitlist;

add_to_waitlist();
```

This makes it clear from the call that the function is not defined locally.

For structs, enums, and other types, the type itself is usually imported:

```rust
use std::collections::HashMap;

let map = HashMap::new();
```

This is a Rust code convention, not a technical restriction of the language. The Rust Programming Language (2)

---

# Name Conflicts and `as`

Two types with the same name cannot be imported directly into the same scope:

```text
std::fmt::Result
std::io::Result
```

One option is to import their parent modules:

```rust
use std::fmt;
use std::io;

fn f1() -> fmt::Result { ... }
fn f2() -> io::Result<()> { ... }
```

Another is to use an alias:

```rust
use std::fmt::Result;
use std::io::Result as IoResult;
```

Now:

```text
Result
IoResult
```

do not conflict. The Rust Programming Language (2)

---

# `pub use` — Re-Exporting

Ordinary:

```rust
use ...
```

creates a private name in the current scope.

But:

```rust
pub use ...
```

makes that name part of the public API.

Example:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;
```

Without re-exporting, the external path might be:

```text
restaurant::front_of_house::hosting::add_to_waitlist
```

After:

```rust
pub use ...
```

you can expose the more convenient path:

```text
restaurant::hosting::add_to_waitlist
```

This separates:

```text
internal library organization
           from
its external public API
```

The Rust Programming Language (2)

---

# External Packages

For an external package, the mechanism is already familiar from Cargo.

First the dependency is listed in:

```toml
[dependencies]
...
```

Cargo then makes the crate available to the project, and required items can be brought into scope with `use`.

The standard library:

```text
std
```

is also a crate, but it does not need to be added to `Cargo.toml` because it ships with Rust. The Rust Programming Language (2)

---

# Grouping `use`

Instead of:

```rust
use std::cmp::Ordering;
use std::io;
```

you can write:

```rust
use std::{cmp::Ordering, io};
```

If one path is part of another:

```rust
use std::io;
use std::io::Write;
```

you can write:

```rust
use std::io::{self, Write};
```

Here `self` means `std::io` itself, while `Write` is an item inside it. The Rust Programming Language (2)

---

## Glob `*`

All public items can be imported with:

```rust
use std::collections::*;
```

But the book recommends caution: after a glob import, it is harder to tell which names are in scope and where they came from.

The book notes `*` as common in tests and sometimes in prelude-style patterns. The Rust Programming Language (2)

---

# Splitting Modules Across Files

The module tree and filesystem structure are related, but **a module is defined through `mod`**, not merely by the existence of a file.

For example:

```rust
mod front_of_house;
```

in:

```text
src/lib.rs
```

tells the compiler to find the module's contents in an external file.

The main form is:

```text
src/front_of_house.rs
```

The book also lists the older supported form:

```text
src/front_of_house/mod.rs
```

The Rust Programming Language (2)

---

## Submodules in Separate Files

Suppose the module tree is:

```text
front_of_house
└── hosting
```

Filesystem structure:

```text
src/
├── lib.rs
├── front_of_house.rs
└── front_of_house/
    └── hosting.rs
```

`src/lib.rs`:

```rust
mod front_of_house;
```

`src/front_of_house.rs`:

```rust
pub mod hosting;
```

`src/front_of_house/hosting.rs`:

```rust
pub fn add_to_waitlist() {}
```

`hosting` is a child of `front_of_house`, so its file is placed in a directory corresponding to the parent module. The Rust Programming Language (2)

---

## `mod` Is Not a Textual Include

An important detail from the book:

```rust
mod front_of_house;
```

does not mean “insert the contents of this file here.”

It declares a module at a specific point in the **module tree**.

Once the file has been included in the tree through `mod`, the rest of the code refers to its items through paths.

So:

```text
files
→ physical placement of code

modules
→ logical structure of code
```

The Rust Programming Language (2)

---

# Binary + Library in One Package

The book specifically recommends a useful structure when a package contains:

```text
src/main.rs
src/lib.rs
```

Most functionality is placed in the library crate, while the binary crate contains just enough code to start the application and call the library portion.

Then:

```text
lib.rs
→ main reusable logic

main.rs
→ application startup
```

The binary crate uses the public API of the library crate similarly to an external user. This helps keep functionality reusable while also testing the quality of the public API. The Rust Programming Language (2)

---

# 4. What to Remember

Main hierarchy:

```text
PACKAGE
→ Cargo.toml
→ contains crates

CRATE
→ compilation unit

MODULE
→ organizes code inside a crate

PATH
→ address of an item in the module tree
```

---

Crate roots:

```text
src/main.rs
→ binary crate root

src/lib.rs
→ library crate root

src/bin/*.rs
→ additional binary crates
```

A package:

```text
at most 1 library crate
any number of binary crates
at least 1 crate
```

---

Modules:

```rust
mod module_name {
    ...
}
```

or:

```rust
mod module_name;
```

when the code is stored in a separate file.

Tree:

```text
crate
└── module
    └── submodule
```

---

Paths:

```rust
crate::module::item
```

→ absolute path inside the current crate.

```rust
module::item
```

→ relative path.

```rust
super::item
```

→ item in the parent module.

---

Privacy:

```text
by default
→ private
```

Expose with:

```rust
pub mod ...
pub fn ...
pub struct ...
pub enum ...
```

But remember:

```text
pub module
≠ everything inside is public
```

For structures:

```text
pub struct
→ fields are still private
  unless marked pub
```

For enums:

```text
pub enum
→ its variants are public
```

---

`use`:

```rust
use crate::some::module;
```

→ short name in the current scope.

`as`:

```rust
use std::io::Result as IoResult;
```

→ rename to resolve a conflict.

`pub use`:

```rust
pub use crate::internal::item;
```

→ re-export and part of the external API.

---

Minimal model of a real project:

```text
Cargo.toml
│
└── src/
    ├── lib.rs
    │     ↓
    │   module tree
    │     ↓
    │   main logic/API
    │
    ├── main.rs
    │     ↓
    │   application startup
    │
    └── modules...
```

Three rules that are especially important not to confuse:

```text
1. A file does not become a module by itself.
   The module must be declared with mod.

2. pub exposes only the item
   directly preceded by pub.

3. use does not move or expose an item.
   It only introduces a convenient name
   into a particular scope.
```

Final chain:

```text
Cargo.toml
   ↓
package
   ↓
crate root
   ↓
module tree
   ↓
paths
   ↓
pub determines visibility
   ↓
use simplifies access
```

**Notes size:** approximately **9–11 thousand characters** including code and diagrams.

**Time to copy into a notebook:** approximately **50–65 minutes**. The most important things to remember months later are the distinction between `package / crate / module`, the meaning of `crate::` and `super::`, the `pub` rules, the purpose of `use`/`pub use`, and the relationship between `mod` and the filesystem layout.
