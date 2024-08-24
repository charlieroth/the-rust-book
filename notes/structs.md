# Structures

A *struct* or *structure* is a custom data type that lets you package together
and name multiple related values that make up a meaningful group.

In order to make a field in struct mutable, the whole instance of the struct
must be mutable. Rust does not allow specific fields to be mutable or
immutable.

## Tuple Structs

Rust supports structs that look similar to tuples, called *tuple structs*.
Tuple structs don't have names associated with their fields, just types.

## Method Syntax

*Methods* are similar to functions: declared with `fn` keyword and a name,
they can have parameters and a return value, they contain some code that's
run when the method is called from somewhere else.

Unlike functions, methods are defined within the context of a struct and
their first parameter is always `self`, which represents the instance
of the struct the method is being called on.

### Where is the `->` Operator?

In C/C++, you use `->` if you're calling the method on a pointer to the
object and need to dereference the pointer first.

Rust does not have an equivalent to the `->` operator. Rust has a feature
called *automatic referencing and dereferencing*.

When you call a method with `object.something()`, Rust automatically adds
in `&`, `&mut`, or `*` so `object` matches the signature of the method. In
other words, the following are the same:

```
p1.distance(&p2);
(&p1).distance(&p2);
```

## Associated Functions

All functions defined within an `impl` block are called *associated functions*
becuase they're associated with the type named after the `impl`.

A struct can have multiple `impl` blocks. In most cases it is not useful to
have multiple `impl` blocks but a common use case is the separate functionality
that is only used in the context of *generic types* and *traits*.
