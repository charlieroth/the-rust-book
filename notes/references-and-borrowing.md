# References and Borrowing

A *reference* is like a pointer in that it's an address that can be followed
to access the data stored at that address; that data is owned by some other
variable. Unlike a pointer, a reference is guaranteed to point to a valid
value of a particular type for the life of that reference.

An example of using a reference. Since the program needs to use the data in
the variable `s1` after the function call `calculate_length`, a reference to
the variable must be passed to the function rather than the variable itself
so that ownership can be retained by the calling function.

```rust
fn main() {
  let s1 = String::from("hello");
  let len = calculate_length(&s1);
  println!("The length of '{s1}' is {len}");
}

fn calculate_length(s: &String) -> usize {
  return s.len();
}
```

## Rules of References

1. At any given time, you can have *either* one mutable reference or *any*
   number of immutable references
2. References must always be valid

## Mutable References

In the case where the program needs to mutate the underlying data of a variable
a *mutable reference* needs to be used.

```rust
fn main() {
  let mut s1 = String::from("hello");
  change(&mut s1);
  println("{s1}");
}

fn change(s: &mut String) {
  s.push_str(", world");
}
```

Mutable references have one restriction: **if you have a mutable reference to a
value, you can have no other reference to that value**. For example:

```rust
let mut s = String::from("hello");
let r1 = &mut s;
      // ------ first mutable borrow occurs here
let r2 = &mut s;
      // ^^^^^^ second mutable borrow occurs here
println!("{}, {}", r1, r2);
                // -- first borrow later used here
```

This rule is in place to prevent *data races* which are similar to a race condition
and happens when these three behaviors occur:

- Two or more pointers access the same data at the same time
- At least one of the pointers is being used to write to the data
- There's no mechanism being used to synchronize access to the data

Similar to the above rule, **there can not be both a immutable reference and a
mutable reference to the same piece of data**

## Dangling References

A *dangling pointer* is a pointer that references a location in memory that
may have been given to another part of the program. The Rust compiler
guarantees that references will never be dangling references: **if you have a
reference to some data, the compiler will ensure the data will not go out
of scope before the reference to that data does**.

```rust
fn main() {
  let reference_to_nothing = dangle(); // the reference we think is being returned
                                       // is not valid anymore due to the local
                                       // being dropped
}

fn dangle() -> &String {
  let s = String::from("hello");
  &s
} // `s` goes out of scope and is freed
```
