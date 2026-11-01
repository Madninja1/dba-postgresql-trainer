# Step 2. Basic Rust Language

## 1. Topic

Rust is a **statically typed** language: the type of every value must be known at compile time. The compiler can often infer the type automatically, but an explicit annotation is required when the type is ambiguous.

The basic layer of the language includes:

```text
variables and constants
        ↓
data types
        ↓
expressions and functions
        ↓
branching
        ↓
loops
```

Rust keywords are reserved by the language and cannot be used as ordinary variable or function names.

---

# 2. Mechanism

The main idea of basic Rust is that **every value always has a concrete type, and many language constructs are expressions and therefore produce values themselves**.

For example:

```rust
let x = 5;
```

Here `5` is an expression that evaluates to a value, while `let x = 5;` is a statement that binds that value to the name `x`.

This distinction is especially important for functions, blocks, `if`, and `loop`.

```rust
let x = {
    let a = 3;
    a + 1
};
```

The block `{ ... }` is an expression. Its final expression `a + 1` evaluates to `4`, so:

```text
x = 4
```

Key point:

```rust
a + 1     // expression → returns a value
a + 1;    // statement → does not return a value
```

A semicolon can change the meaning of the code.

---

# 3. Important Details

## Variables, `mut`, Constants, and Shadowing

Variables are immutable by default:

```rust
let x = 5;
```

After that you cannot do:

```rust
x = 6; // error
```

A mutable variable requires `mut`:

```rust
let mut x = 5;
x = 6;
```

Immutability allows the compiler to guarantee that a value will not change unexpectedly. `mut` makes it explicit that mutation is intended by the program.

### Constants

A constant is declared with `const`:

```rust
const MAX_SIZE: u32 = 100;
```

It differs from an ordinary immutable variable in these ways:

```text
let                     const
────────────────────────────────────────
immutable by default    always immutable
type often inferred     type is mandatory
runtime value allowed   only a constant expression
ordinary scope          can also be declared globally
```

`mut const` is not allowed.

Constant names are conventionally written as:

```text
UPPER_CASE_WITH_UNDERSCORES
```

Constants are suitable for values that have the same meaning in many parts of a program.

---

### Shadowing

Rust allows you to create a **new variable with the same name**:

```rust
let x = 5;
let x = x + 1;
```

The second `x` shadows the first one.

Inside a nested scope this works locally:

```rust
let x = 6;

{
    let x = x * 2; // 12
}

// x = 6 is available again
```

The main difference from `mut`:

```text
mut
→ changes the value of an existing variable
→ the variable's type cannot change

shadowing with let
→ creates a new variable
→ the type may change
```

Therefore this is valid:

```rust
let spaces = "   ";
let spaces = spaces.len();
```

The first `spaces` is a string, the second is a number.

But this is invalid:

```rust
let mut spaces = "   ";
spaces = spaces.len(); // different type
```

**Rule:** use `mut` to change a value; use shadowing to transform a value while keeping a convenient name.

---

## Data Types

Rust must know the type of every value at compile time.

Type inference often works:

```rust
let x = 5;
```

But when the type is ambiguous, it must be specified explicitly:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

The main types in this chapter are divided into:

```text
scalar
├── integer
├── floating point
├── bool
└── char

compound
├── tuple
└── array
```

### Integers

Signed:

```text
i8  i16  i32  i64  i128  isize
```

Unsigned:

```text
u8  u16  u32  u64  u128  usize
```

`i` means signed integer, `u` means unsigned integer.

`isize` and `usize` depend on the architecture: 32 bits on a 32-bit system and 64 bits on a 64-bit system.

`usize` is especially common when indexing collections.

If the type of an integer literal is not otherwise constrained, the default is:

```rust
i32
```

A type can be specified with a suffix:

```rust
57u8
```

Different literal forms are supported:

```rust
98_222       // decimal
0xff         // hexadecimal
0o77         // octal
0b1111_0000  // binary
b'A'         // byte, u8
```

`_` inside a number is only for readability:

```rust
1_000 == 1000
```

### Integer Overflow

If a value exceeds the range of its type:

```text
debug build
→ overflow checking
→ panic

release build
→ value wraps
```

Relying on automatic wrapping is considered a bug.

For controlled behavior, integer types provide method families:

```text
wrapping_*    → wrapping
checked_*     → None on overflow
overflowing_* → value + overflow flag
saturating_*  → clamp to min/max value
```

---

### Floating-Point Numbers

There are two types:

```text
f32
f64
```

The default is:

```rust
f64
```

`f32` is single precision, `f64` is double precision.

Both follow IEEE-754.

---

### Numeric Operations

Main operations:

```rust
+  // addition
-  // subtraction
*  // multiplication
/  // division
%  // remainder
```

With integer division, the fractional part is truncated toward zero:

```rust
-5 / 3 // -1
```

---

### `bool`

Boolean type:

```rust
bool
```

It has two values:

```rust
true
false
```

Its size is one byte.

It is especially important for conditions.

---

### `char`

```rust
let c = 'z';
let c: char = 'ℤ';
```

`char`:

```text
size = 4 bytes
represents a Unicode scalar value
```

Therefore it is not limited to ASCII.

It is important to distinguish:

```rust
'a'   // char
"a"   // string literal
```

What a person perceives as one visible character does not always correspond to one `char`; this is covered in more detail together with strings.

---

## Tuples and Arrays

### Tuple

A tuple combines several values whose types may differ:

```rust
let t: (i32, f64, u8) = (500, 6.4, 1);
```

A tuple has a fixed size.

Values can be accessed by index:

```rust
t.0
t.1
t.2
```

Or by destructuring:

```rust
let (x, y, z) = t;
```

The special empty tuple:

```rust
()
```

is called the **unit type**.

It represents the absence of any other return value. If an expression does not produce another value, its result is `()`.

---

### Array

An array:

```rust
let a = [1, 2, 3, 4, 5];
```

Unlike a tuple:

```text
tuple → element types may differ
array → all elements have one type
```

The array length is fixed.

Array type:

```rust
let a: [i32; 5];
```

means:

```text
element type = i32
length = 5
```

Repeat one value:

```rust
let a = [3; 5];
```

is equivalent to:

```rust
[3, 3, 3, 3, 3]
```

Access:

```rust
a[0]
a[1]
```

Indexing starts at `0`.

If an index is outside the array bounds at runtime, Rust terminates the program with a panic instead of reading an invalid memory location.

```text
index < length  → access
index >= length → panic
```

An array is appropriate when the number of elements is known in advance and should not change. For a variable-sized collection, the book later introduces `Vec<T>`.

---

## Functions and Expressions

A function is defined with `fn`.

Function and variable names are conventionally written in `snake_case`.

```rust
fn calculate_value() {
}
```

A function may appear before or after the place where it is called in the source file, as long as it is available from the relevant scope.

### Parameters

The type of every function parameter must be specified:

```rust
fn print_value(x: i32) {
}
```

Several parameters:

```rust
fn measurement(value: i32, unit: char) {
}
```

The function signature explicitly tells the compiler the expected types.

---

### Statements and Expressions

This is one of the most important rules of basic Rust.

A **statement** performs an action and does not return a value:

```rust
let x = 5;
```

An **expression** evaluates to a value:

```rust
5 + 5
```

Expressions also include:

```text
function call
macro call
block { ... }
if
loop
```

Example:

```rust
let y = {
    let x = 3;
    x + 1
};
```

`y == 4`.

The final line of the block:

```rust
x + 1
```

does not end with `;`, so its value becomes the value of the whole block.

---

### Function Return Value

The result type is written after `->`:

```rust
fn plus_one(x: i32) -> i32 {
    x + 1
}
```

The final expression of a function is returned implicitly.

That is:

```rust
x + 1
```

not:

```rust
x + 1;
```

The second form returns `()` because the expression was turned into a statement.

An explicit early return is possible:

```rust
return value;
```

But the book notes that functions usually return their final expression.

---

## Conditions and Loops

### `if`

```rust
if condition {
    ...
} else {
    ...
}
```

The condition must have type:

```rust
bool
```

Rust does not perform automatic truthy/falsy conversion:

```rust
if number { }      // invalid

if number != 0 { } // valid
```

In a chain:

```rust
if ...
else if ...
else if ...
else ...
```

**the first branch whose condition is true** is executed; later conditions are no longer checked.

---

### `if` as an Expression

Because `if` is an expression:

```rust
let x = if condition { 5 } else { 6 };
```

branching can directly produce a value.

But every possible result branch must produce a compatible type:

```rust
let x = if condition {
    5
} else {
    6
};
```

is valid.

But:

```rust
let x = if condition {
    5
} else {
    "six"
};
```

does not compile.

Reason: `x` must have one type known to the compiler.

---

### `loop`

`loop` runs code indefinitely:

```rust
loop {
    ...
}
```

Stop it with:

```rust
break;
```

Skip the rest of the current iteration:

```rust
continue;
```

A Rust-specific feature is that `loop` can return a value:

```rust
let result = loop {
    if condition {
        break 10;
    }
};
```

Here the value of the `loop` is `10`.

---

### Loop Labels

With nested loops, ordinary `break` and `continue` refer to the innermost loop.

A loop can be labeled:

```rust
'outer: loop {
    loop {
        break 'outer;
    }
}
```

A label begins with `'`.

It lets you specify exactly which nested loop should be broken or continued.

---

### `while`

Used while a condition remains true:

```rust
while number != 0 {
    number -= 1;
}
```

Mechanically:

```text
check bool
    ↓ true
execute body
    ↓
check again

false
    ↓
exit
```

This is a convenient form of the common `loop + if + break` combination.

---

### `for`

For iterating over a collection, prefer:

```rust
for element in array {
    ...
}
```

instead of manually increasing an index with `while`.

Reasons given in the book:

```text
shorter
safer
less risk of going out of bounds
no need to manually synchronize index and length
```

For this reason, `for` is the most commonly used loop construct among those covered here.

---

### Ranges

The range:

```rust
1..4
```

contains:

```text
1, 2, 3
```

The upper bound is excluded:

```text
start..end
→ start ... end - 1
```

A range can be iterated over:

```rust
for number in 1..4 {
}
```

Reverse the order:

```rust
for number in (1..4).rev() {
}
```

---

### Comments

An ordinary comment begins with:

```rust
// comment
```

and continues to the end of the line.

For multiple lines:

```rust
// first line
// second line
```

Documentation comments are covered later in the book, so they are not examined in detail here.

---

# 4. What to Remember

If the whole chapter is compressed heavily, the core is:

```text
1. Rust is statically typed.
   The type of every value is known at compile time.

2. let creates a variable.
   It is immutable by default.

3. mut allows changing the value of an existing variable.

4. Repeating let creates a new variable:
   this is shadowing.
   Shadowing can even change the type.

5. const:
   always immutable,
   type is mandatory,
   value must be a constant expression.

6. Main scalar types:
   integer, float, bool, char.

7. Main compound types:
   tuple and array.

8. Tuple:
   fixed size,
   element types may differ.

9. Array:
   fixed size,
   all elements have one type,
   out of bounds → panic.

10. Rust distinguishes statements and expressions.

    expression → evaluates to a value
    statement  → does not return a value

11. A semicolon can turn a returning expression
    into a statement.

12. Function:
    fn name(param: Type) -> ReturnType

    Parameter types are mandatory.
    The final expression is usually the function result.

13. An if condition must be bool.
    There is no automatic truthy/falsy conversion.

14. if is an expression:
    let x = if condition { 1 } else { 2 };

    All result branches must have one type.

15. Loops:
    loop  → infinite/manual control
    while → while condition is true
    for   → iterate over collection/range

16. break     → exit
    continue  → next iteration

17. loop can return a value:
    break value;

18. Nested loops can be addressed with labels.

19. For collections, generally prefer for
    to manual indexing with while.

20. The range start..end excludes end.
```

The most important part of this chapter is not the list of types, but three rules that will appear constantly later:

```rust
// 1. Immutability by default
let x = 5;

// 2. Blocks and control-flow constructs can be expressions
let x = if condition { 5 } else { 10 };

// 3. The final expression without ; returns a value
fn f() -> i32 {
    5
}
```

The next step — **ownership** — builds directly on these ideas, where values, variables, scopes, and passing data between functions become the central mechanism of Rust.

**Size:** approximately **10–12 thousand characters** including code.
**Time to copy into a notebook:** approximately **50–65 minutes**; with heavily shortened examples — about **45–55 minutes**.
