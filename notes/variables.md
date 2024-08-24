# Variables

Variables in Rust are immutable by default.

Adding the `mut` keyword to a variable declaration creates a mutable variable.

```rust
let mut x = 5;
println!("x is: {}", x);
x = 6;
println!("x is: {}", x);
```

*Constants* are always immutable and must always be type annotated.
