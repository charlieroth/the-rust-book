# Functions

Rust code uses *snake case* as the conventional styles for function names.

In function signatures you *must* declare the type of each parameter.

Rust is an expression-based language which is an important distinction to
understand. Statements and expressions affect the bodies in functions in
different ways.

- **Statements** are instructions that perform some action and do not return a value
- **Expressions** evaluate to a resultant value

The expression:

```rust
let y = {
  let x = 3;
  x + 1
};
```
is an example of an expression with the last line not having a semicolon. When
you see this in code, this is the same as `return x + 1` but can only be used
when it is on the last line of an expression. This is called *implicit return*.

