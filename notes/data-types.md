# Data Types

Every value in Rust of a certain *data type*.

The two data type subsets are *scalar* and *compound*

## Scalar Types

A *scalar* type represents a single value.

Four primary scalar types:

- Integers
- Floating-point numbers
- Booleans
- Characters

## Compound Types

*Compound* types can group multiple values into one type.

Rust has two primitive compound types:

- Tuples
- Arrays

When you attempt to access an element using indexing, Rust will check that the
index you've specified is less than the array length. In many low-level languages,
this kind of check is not done, and when you provide an incorrect index,
invalid memory can be accessed. Rust protects you against this kind of error by
immediately exiting instead of allowing the memory access and continuing.
