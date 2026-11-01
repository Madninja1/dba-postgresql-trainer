# Step 14. Smart Pointers and Concurrency

## 1. Topic

This step brings together two major topics:

```text
smart pointers
→ manage ownership and access to data

concurrency
→ allows multiple threads
  to execute code at the same time
```

An ordinary reference:

```rust
&T
```

only borrows a value.

A smart pointer is a structure that behaves like a pointer but can additionally own data, store metadata, and provide special behavior. The `Deref` and `Drop` traits are especially important for smart pointers. `String` and `Vec<T>` also belong to this broader idea. The Rust Programming Language(2)

The main types in this step:

```text
Box<T>
→ one owner
→ data on the heap

Rc<T>
→ multiple owners
→ one thread

RefCell<T>
→ borrowing is checked at runtime
→ interior mutability

Weak<T>
→ non-owning reference
→ breaks Rc cycles

Mutex<T>
→ protected mutable access

Arc<T>
→ multithreaded shared ownership
```

And threads communicate through:

```text
thread::spawn
JoinHandle
channels
Mutex<T>
Arc<T>
Send
Sync
```

---

# 2. Mechanism

## `Box<T>`

`Box<T>` stores the value itself on the heap, while a pointer to it remains where the `Box` value is stored.

```rust
let b = Box::new(5);
```

Model:

```text
stack                   heap

b ────────────────────→ 5
```

When a `Box` goes out of scope, both the pointer and the heap data are freed.

One of its main use cases is a type whose size cannot be determined directly from a recursive definition. The Rust Programming Language(2)

---

## Recursive types

This definition is impossible:

```rust
enum List {
    Cons(i32, List),
    Nil,
}
```

Reason:

```text
List contains List
     contains List
          contains List
               ...
```

The compiler cannot calculate a finite size for `List`.

`Box<T>` has a known size regardless of the size of `T`, so it can break the recursion:

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

Now:

```text
List
├─ i32
└─ Box<List>
      ↓
   next List on the heap
```

`List` itself now has a calculable size because it contains a fixed-size pointer rather than another `List` directly.

---

# `Deref`

A smart pointer can behave like an ordinary reference thanks to the trait:

```rust
Deref
```

For example:

```rust
let x = 5;
let y = Box::new(x);

assert_eq!(5, *y);
```

The operator:

```rust
*
```

accesses the value to which the `Box` points.

For a custom type, the concept looks like this:

```rust
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

With `*value`, Rust effectively uses `Deref::deref` to obtain an ordinary reference that it already knows how to dereference. The Rust Programming Language(2)

---

## Deref coercion

Rust can automatically convert a reference to a smart pointer into a reference to its inner type.

For example, if a function accepts:

```rust
fn hello(name: &str) {
    ...
}
```

you can pass:

```rust
let name = Box::new(String::from("Rust"));

hello(&name);
```

Rust can perform the chain:

```text
&Box<String>
     ↓ Deref
&String
     ↓ Deref
&str
```

This is called **deref coercion**. The conversions are determined at compile time and do not require manual chains of `*` and `&`. The Rust Programming Language(2)

There is an analogous `DerefMut` for mutable references.

---

# `Drop`

`Drop` defines automatic cleanup code for a value.

```rust
impl Drop for MyType {
    fn drop(&mut self) {
        ...
    }
}
```

Rust calls `drop` automatically when a value goes out of scope.

It is used to release resources such as:

```text
heap memory
files
network connections
locks
...
```

For example, `Box<T>` uses this mechanism to release its heap data. The Rust Programming Language(2)

Model:

```text
value leaves scope
        ↓
Drop::drop(...)
        ↓
resource cleanup
```

So the ownership/RAII model applies to more than just memory.

---

# `Rc<T>` — multiple owners

Ordinary ownership assumes a single owner.

Sometimes, however, the same data must belong to several parts of a structure at the same time.

For a single-threaded case:

```rust
Rc<T>
```

`Rc` means **reference counted**.

Example:

```rust
let a = Rc::new(value);

let b = Rc::clone(&a);
let c = Rc::clone(&a);
```

Model:

```text
a ─┐
b ─┼──→ DATA
c ─┘
```

Internally, a strong-reference counter is stored:

```text
Rc::new
→ strong_count = 1

Rc::clone
→ +1

Rc dropped
→ -1

strong_count == 0
→ DATA is dropped
```

`Rc::clone` does not deeply copy the data; it increments the reference count. The Rust Programming Language(2)

---

## What `Rc<T>` provides

The key point:

```text
Rc<T>
→ shared ownership
```

But ordinary `Rc<T>` mainly provides shared read access.

Multiple owners must not automatically imply arbitrary simultaneous mutable references, because that would violate borrowing rules. The Rust Programming Language(2)

`Rc<T>` is intended for **single-threaded** use.

---

# `RefCell<T>`

`RefCell<T>` moves borrowing checks from:

```text
ordinary & / &mut
→ compile time

RefCell<T>
→ runtime
```

To obtain an immutable borrow:

```rust
value.borrow()
```

The result is:

```text
Ref<T>
```

For a mutable borrow:

```rust
value.borrow_mut()
```

The result is:

```text
RefMut<T>
```

Both types implement `Deref`, so they can be used similarly to references. The Rust Programming Language(2)

---

## The rules do not change

`RefCell<T>` allows:

```text
multiple Ref<T>

OR

one RefMut<T>
```

So the old rule:

```text
many readers
OR
one writer
```

still applies.

Only the time of the check changes.

With ordinary references, a violation means:

```text
→ compiler error
```

With `RefCell<T>`:

```text
→ the code compiles
→ the violation is detected at runtime
→ panic
```

For example, two active `borrow_mut()` calls cause a `BorrowMutError`. The Rust Programming Language(2)

---

# Interior mutability

`RefCell<T>` allows the inner value to be changed even through an API that externally uses an immutable reference.

This is called:

```text
interior mutability
```

A practical combination:

```text
Rc<RefCell<T>>
```

means:

```text
Rc
→ multiple owners

RefCell
→ interior mutation
```

The result:

```text
several parts of the program
        ↓
share ownership of the data
        ↓
can mutate the inner value
        ↓
borrowing rules are checked at runtime
```

The book demonstrates this combination using a shared portion of multiple lists. The Rust Programming Language(2)

---

# `Rc<T>` cycles

Reference counting has an important risk.

If:

```text
A owns Rc<B>
B owns Rc<A>
```

you get:

```text
A ──strong──→ B
↑             │
└──strong─────┘
```

Even after external owners disappear:

```text
A strong_count > 0
B strong_count > 0
```

neither object will be dropped.

The result is:

```text
memory leak
```

Ownership and the borrow checker do not guarantee the absence of such logical cycles.

---

# `Weak<T>`

For a reference that **must not own the object**, use:

```rust
Weak<T>
```

Create it from `Rc<T>`:

```rust
let weak = Rc::downgrade(&strong);
```

This:

```text
does not increase strong_count
increases weak_count
```

Therefore, `Weak<T>` does not keep the object alive. The Rust Programming Language(2)

To obtain the object again:

```rust
weak.upgrade()
```

returns:

```text
Option<Rc<T>>
```

Why `Option`?

Because the object may already have been destroyed:

```text
still exists
→ Some(Rc<T>)

already dropped
→ None
```

---

## Typical `Weak` example

For a tree:

```text
parent
  │
  └── strong Rc ──→ child

child
  │
  └── Weak ───────→ parent
```

The parent owns the child.

The child only knows about the parent but **does not own it**.

This breaks the cycle and allows the structure to be dropped correctly. The Rust Programming Language(2)

---

# Moving to threads

A thread is a separate sequence of execution inside a process.

Rust considers several concurrency models:

```text
threads
→ concurrent execution

message passing
→ transferring values between threads

shared state
→ shared access to data

Send / Sync
→ type-level guarantees
```

The Rust Programming Language(2)

---

# `thread::spawn`

Create a new thread:

```rust
use std::thread;

let handle = thread::spawn(|| {
    // code in the new thread
});
```

`spawn` receives a closure.

Thread execution order is not guaranteed in advance:

```text
main thread ────────┐
                    │ concurrent
spawned thread ─────┘
```

The main thread can finish before the spawned thread, so `spawn` by itself does not guarantee that the spawned thread will complete its work.

---

# `JoinHandle` and `join`

`thread::spawn` returns:

```text
JoinHandle
```

You can call:

```rust
handle.join().unwrap();
```

`join` blocks the current thread until the corresponding thread finishes. The Rust Programming Language(2)

Model:

```text
spawn
  ↓
JoinHandle
  ↓
join()
  ↓
wait until thread finished
```

Where `join` is placed matters.

If you call `join` first and only then do the main-thread work, those code sections will no longer truly interleave.

---

# `move` with threads

A new thread can live longer than the scope in which it was created.

So this code is conceptually unsafe:

```rust
let v = vec![1, 2, 3];

thread::spawn(|| {
    println!("{v:?}");
});
```

The thread could retain a reference to `v` after `v` had already been dropped.

Use:

```rust
thread::spawn(move || {
    println!("{v:?}");
});
```

`move` transfers ownership of the required values into the new thread's closure. The Rust Programming Language(2)

---

# Channels

Instead of mutating shared data, threads can **send messages**.

Create a channel:

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();
```

Where:

```text
tx
→ transmitter
→ sender

rx
→ receiver
```

Example:

```rust
thread::spawn(move || {
    let msg = String::from("hi");

    tx.send(msg).unwrap();
});

let received = rx.recv().unwrap();
```

The Rust Programming Language(2)

---

## `send` transfers ownership

```rust
tx.send(value)
```

moves ownership of a non-`Copy` value into the channel.

After:

```rust
tx.send(val).unwrap();
```

the sender can no longer use:

```rust
val
```

Model:

```text
producer owns value
        ↓ send
channel owns/transfers value
        ↓ receive
consumer owns value
```

This is an important guarantee: the sender cannot continue mutating an object at the same time after it has already transferred that object to another thread. The Rust Programming Language(2)

---

## `recv` and `try_recv`

```rust
rx.recv()
```

blocks the thread:

```text
no message
→ wait

message arrives
→ Ok(value)

channel closed
→ Err(...)
```

`try_recv()`:

```text
does not block
→ immediately returns Result
```

It is useful when a thread must continue doing other work and only periodically check the channel. The Rust Programming Language(2)

`rx` can also be used as an iterator, receiving messages until the channel closes.

---

# `Mutex<T>`

With the shared-state approach, multiple threads work with the same data.

For controlled mutable access:

```rust
Mutex<T>
```

Example:

```rust
let m = Mutex::new(5);

{
    let mut value = m.lock().unwrap();
    *value = 6;
}
```

`lock()`:

```text
tries to acquire the lock
        ↓
if it is already held
→ current thread waits

if acquired
→ access to data
```

`lock()` returns a `MutexGuard` wrapped in a `LockResult`. `MutexGuard` implements `Deref`, so it behaves like a reference to the inner value. The Rust Programming Language(2)

---

## Automatic unlocking

`MutexGuard` also implements:

```text
Drop
```

When the guard goes out of scope:

```text
MutexGuard dropped
      ↓
mutex unlocked
```

You do not have to remember to unlock it manually. The Rust Programming Language(2)

This connects to the first half of the step:

```text
Drop
→ automatic resource management

resource here
→ mutex lock
```

---

# Why `Mutex<T>` alone is not enough

If one `Mutex<T>` must be shared among several threads, shared ownership becomes necessary.

You might think of:

```text
Rc<Mutex<T>>
```

But `Rc<T>` is not thread-safe.

Its reference counter is not designed to be changed concurrently from different threads. Therefore `Rc<T>` does not implement `Send`, and the compiler does not allow it to be transferred between threads. The Rust Programming Language(2)

---

# `Arc<T>`

The multithreaded counterpart of `Rc<T>` is:

```rust
Arc<T>
```

`A` means:

```text
atomic
```

Usage:

```rust
let counter =
    Arc::new(Mutex::new(0));
```

For each thread:

```rust
let counter =
    Arc::clone(&counter);

let handle =
    thread::spawn(move || {
        let mut num =
            counter.lock().unwrap();

        *num += 1;
    });
```

The result:

```text
Arc
→ shared ownership between threads

Mutex
→ exclusive mutable access
```

So the classic combination is:

```text
Arc<Mutex<T>>
```

The Rust Programming Language(2)

---

# Relationship between `Rc<RefCell<T>>` and `Arc<Mutex<T>>`

A very useful model:

```text
ONE THREAD

Rc<T>
→ multiple ownership

RefCell<T>
→ interior mutability

Rc<RefCell<T>>
→ shared mutable data
```

And in a multithreaded case:

```text
MULTIPLE THREADS

Arc<T>
→ thread-safe multiple ownership

Mutex<T>
→ synchronized interior mutability

Arc<Mutex<T>>
→ shared mutable state
```

The book directly compares these two pairs. The Rust Programming Language(2)

---

# `Send`

`Send` is a marker trait.

It means:

```text
ownership of a value of this type
can safely be transferred
from one thread to another
```

Most Rust types are `Send`.

But, for example:

```text
Rc<T>
→ not Send
```

because its reference counter is not thread-safe.

If a type is composed entirely of `Send` types, it usually receives `Send` automatically. The Rust Programming Language(2)

---

# `Sync`

`Sync` means:

```text
&T
can safely be used
from multiple threads
```

The book states the relationship as:

```text
T: Sync
if
&T: Send
```

Examples:

```text
Rc<T>
→ not Sync

RefCell<T>
→ not Sync

Mutex<T>
→ Sync
```

`RefCell<T>` is not suitable for multithreaded shared access because its runtime borrowing check is not thread-safe. The Rust Programming Language(2)

---

# 3. Important details

## `Box`, `Rc`, and `RefCell` solve different problems

Do not treat them as interchangeable containers.

```text
Box<T>
→ one owner
→ heap
→ fixed-size pointer

Rc<T>
→ multiple owners
→ single-thread

RefCell<T>
→ one direct owner,
  but borrowing is checked at runtime
```

The combination:

```text
Rc<RefCell<T>>
```

is needed when you require both:

```text
shared ownership
+
mutation
```

in single-threaded code.

---

## Compile-time vs runtime borrowing

Ordinary references:

```text
borrowing error
→ program does not compile
```

`RefCell<T>`:

```text
borrowing error
→ program compiles
→ panic at runtime
```

Therefore `RefCell<T>` expands the set of models you can express, but moves some failures to runtime. The Rust Programming Language(2)

---

## `Rc` can create memory leaks

Rust guarantees memory safety, but a reference cycle through `Rc<T>` can logically keep data alive forever.

For relationships that are not ownership:

```text
Rc<T>
→ strong ownership

Weak<T>
→ non-owning relation
```

`Weak<T>` is used specifically to break such cycles.

---

## Threads do not guarantee execution order

After `spawn`, you cannot rely on a specific interleaving of instructions between threads.

To guarantee completion:

```rust
handle.join()
```

But `join` blocks the current thread, so its placement affects how much concurrent execution actually happens. The Rust Programming Language(2)

---

## Message passing and shared state are different models

Through a channel:

```text
transfer ownership of data
→ to another thread
```

With:

```text
Arc<Mutex<T>>
```

multiple threads continue working with one shared object.

Both models preserve the fundamental ownership rules.

---

## `Mutex<T>` does not eliminate every error

It helps prevent unsynchronized access to data, but logical errors such as a **deadlock** are still possible:

```text
thread A
owns lock 1
waits for lock 2

thread B
owns lock 2
waits for lock 1

→ both wait forever
```

Rust cannot automatically rule out every such scenario. The Rust Programming Language(2)

---

## `Arc` should not always replace `Rc`

`Arc` uses atomic reference counting, which is necessary for thread safety but has additional cost.

Therefore:

```text
single thread
→ Rc<T>

multiple threads
→ Arc<T>
```

when reference-counted shared ownership is what you need. The Rust Programming Language(2)

---

# 4. What to remember

The main smart-pointer map:

```text
Box<T>
→ heap
→ one owner
→ recursive types

Rc<T>
→ many owners
→ one thread

RefCell<T>
→ borrowing at runtime
→ interior mutability

Weak<T>
→ non-owning Rc reference
→ breaks reference cycles
```

---

Traits:

```text
Deref
→ smart pointer behaves like reference

Drop
→ automatic cleanup on scope exit
```

Deref coercion:

```text
&Box<String>
→ &String
→ &str
```

can happen automatically.

---

Remember the combination:

```text
Rc<RefCell<T>>
```

as:

```text
multiple ownership
+
mutable inner value
+
single thread
```

---

Threads:

```rust
let handle = thread::spawn(move || {
    ...
});

handle.join().unwrap();
```

Read it as:

```text
spawn
→ create a thread

move
→ transfer the required values
  into the closure's ownership

join
→ wait for completion
```

---

Channels:

```rust
let (tx, rx) = mpsc::channel();

tx.send(value);
rx.recv();
```

Model:

```text
producer
  │
  │ move value
  ▼
channel
  │
  ▼
consumer
```

`recv` waits.

`try_recv` does not wait.

---

Shared mutable state:

```rust
Arc<Mutex<T>>
```

Read it as:

```text
Arc
→ multiple threads own the value

Mutex
→ one thread at a time
  receives mutable access
```

Main pattern:

```rust
let data =
    Arc::new(Mutex::new(value));

let data =
    Arc::clone(&data);

thread::spawn(move || {
    let mut value =
        data.lock().unwrap();

    // modify *value
});
```

---

Marker traits:

```text
Send
→ ownership can be transferred
  between threads

Sync
→ references to a value can
  safely be used
  from multiple threads
```

In practice:

```text
Rc<T>
→ single-thread sharing

Arc<T>
→ multi-thread sharing

RefCell<T>
→ single-thread interior mutability

Mutex<T>
→ multi-thread synchronized mutability
```

---

The most important final diagram for this step:

```text
                    OWNERSHIP
                        │
        ┌───────────────┴───────────────┐
        │                               │
     one owner                     many owners
        │                               │
     Box<T>                      Rc<T> / Arc<T>
                                        │
                         ┌──────────────┴─────────────┐
                         │                            │
                     one thread                 many threads
                         │                            │
                     Rc<T>                        Arc<T>


                    MUTABILITY
                        │
             ┌──────────┴──────────┐
             │                     │
          one thread           many threads
             │                     │
         RefCell<T>             Mutex<T>
```

And two combinations worth memorizing literally:

```text
Rc<RefCell<T>>
→ shared mutable state
→ single-thread

Arc<Mutex<T>>
→ shared mutable state
→ multi-thread
```

The central idea of both chapters is the same: **Rust does not discard ownership when moving to complex data structures or threads. `Box`, `Rc`, `RefCell`, `Arc`, `Mutex`, channels, `Send`, and `Sync` simply provide different ways to express ownership and access so that the type system can preserve safety guarantees.**

**Note length:** approximately **13–15 thousand characters** including code and diagrams.

**Time to copy into a notebook:** approximately **75–90 minutes**. This is one of the two densest steps in the course; if shortening it, you can first omit the custom `Deref` implementation and details of `Weak::strong_count/weak_count`, but keep `Box`, `Rc<RefCell<T>>`, `Rc` cycles, `spawn/join`, channels, `Arc<Mutex<T>>`, `Send`, and `Sync`.
