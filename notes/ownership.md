# Ownership

Ownership is Rust's most unique feature and has deep implications
for the programs written. It enables memory safety guarantees
without garbage collection, so it is important to understand how it
works.

*Ownership* is a set of rules that govern how a program manages
memory. The two traditional approaches to managing memory are:

1. **Manually** managing with explicit allocations and deallocations
   of memory
2. **Automatic** memory management via garbage collection

Rust uses a third approach where memory is managed through a system
of ownership with a set of rules the compiler checks. If any of
these rules are violated, the program won't compile.

## Stack and Heap memory

In Rust, whether a value is on the Stack or the Heap affects
how the language behaves and might force you to make certain
decisions about how you write your program.

The Stack stores values in the order it gets them and removes them
in the opposite order, this is known as LIFO (last in, first out).
All data stored on the stack must have a known, fixed size.

The Heap stores values in a more random behavior. Data with an
unknown size at compile time or a size that might change must
be store on the Heap. The memory allocator will find an empty spot
according to how memory memory is bring requested, marks it as
being used and returns a *pointer*, which is the address of the
location.

Pushing a value onto the Stack is faster than allocating a value
onto the Heap. Accessing data on the Heap is slower than accessing
data on the Stack.

Ownership addresses the problems with keeping track of what parts
of code are using what data on the Heap, minimizing the amount of
duplicate data on the Heap and cleaning up unused data on the Heap
so the program does not run out or memory.

## Ownership Rules

1. Each value in Rust has an *owner*.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

In the snippet below, to ensure memory safety, after the line
`let s2 = s1;`, Rust considers `s1` as no longer valid. This
is to prevent a double free when both variables go out of scope
since they both point the same piece of data on the Heap.

```rust
let s1 = String::from("hello");
//  -- move occurs because `s1` has type `String`, which does not implement
// the `Copy` trait
let s2 = s1;
//       -- value moved here
println!("{s1}, world!");
// ^^^^ value borrowed here after move
```

### Variables and Data Interacting with Clone

If your program requires a deep copy of the Heap data of the `String`, not just
the Stack data, the `clone` method can be used. When the see the `clone` method
being used, it is a visual indicator that something different is going on.
The use of `clone` is discouraged but cannot always be avoided. A healthy
mindset with the use of the `clone` method is to try to write your program
without it and if you can not figure out how to write the program without the
use of `clone`, then use `clone`.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
println!("{s1}, world");
```

### Stack-Only Data: Copy

A type can be implement the `Copy` trait and will be stored on the Stack.

A type can not implement both the `Copy` and `Drop` trait.

Some types that implement the `Copy` trait are:

- Integer types
- Boolean type
- Floating-point types
- Character type
- Tuples, if they only contain types that also implement `Copy`
  - `(i32, i32)` implements `Copy`, `(i32, String)` does not

## Ownership and Functions

Passing a variable to a function will *move* or *copy*, just as assignment does

If a variable that implements `Drop`, such as `String`, is passed to a function
then that function retains ownership of that variable's underlying data. When
the function returns, that newly owned data is now out of scope and `drop()`
is called on the variable, thus freeing the memory. If the call site tries
to use that variable after the function call this will result in a compile time
error.

```rust
fn main() {
  let s = String::from("hello");
  takes_ownership(s); // `s` value moves into the function
  // `s` is no longer valid at this point

  let x = 5;
  makes_copy(x); // `x` is `Copy` therefore it's value does
                 // not move into the function
  // `x` is still valid at this point
}

fn takes_ownership(s: String) { // `s` comes into scope
  println("{s} world!");
} // Here `s` goes out of scope and `drop` is called, backing memory is freed

fn makes_copy(x: i32) {
  println("The value of x is: {x}");
} // Here `x` goes out of scope, nothing special happens
```

## Return Values and Scope

Returning values can also transfer ownership. For example...

```rust
fn main() {
  let s1 = gives_ownership();
  let s2 = String::from("hello");
  let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
  let s = String::from("yours");
  return s;
}

fn takes_and_gives_back(s: String) -> String {
  return s;
}
```

This ceremony of taking and giving ownership is not always desired, another
way to interact with owned data is via *References*
