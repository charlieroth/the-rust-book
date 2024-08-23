# Handling Potential Failure with `Result`

Rust's choice of error handling comes in the form of an enumeration type
called the `Result` type

```rust
pub enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

All error handling in Rust is built on top of this enumeration, unifying error handling
for all classes of errors
