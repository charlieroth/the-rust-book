# Slices

*Slices* reference a continguous sequence of elements in a **collection**
rather than the whole collection. A slice is a kind of reference, so it does
not have ownership.

## String Slices

A *string slice* is a reference to a part of a `String`

```rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```
