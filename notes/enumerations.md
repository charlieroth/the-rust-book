# Enums and Pattern Matching

Enums allow you to define a type by enumerating its possible *variants*.

Enums can be as simple as a type defining possible variants like:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

Enums can also be defined as a variant with some associated data:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr(String::from("127.0.0.1"));
```

where the associated data represents the address for that variant. This
highlights a property of enum variants. An enum variant can become a function
that constructs an instance of the enum.

Enums with variant payloads can also have different data types for the
associated data:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

Enum variant payloads are not limited to primitive data types, they can
also contain custom data types:


```rust
struct IpV4Addr {
  // ...
}

struct IpV6Addr {
  // ...
}

enum IpAddr {
    V4(IpV4Addr),
    V6(IpV6Addr),
}
```

this functionality allows each variant to have it's own functionality (state)
allowing a program to have very defined behavior depending on which variant
of that enum a variable holds.

A common use case of associated payloads on an enum type is for handling
different kinds of network messages. Where each variant is a structure,
in the case of `Quit` the message is a *unit struct* and in the cases of
`Write` and `ChangeColor` the message is a *tuple struct* and finally the
`Move` message is just a *struct* with fields `x` and `y`.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

Enums, like *structs*, can also have associated methods defined in `impl`
blocks

```rust
impl Message {
    fn call(&self) {
        // ...
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

## The `Option` Enum and Its Advantages Over Null Values

The `Option` enum in the standard library, defined as:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

encodes the common scenario in which a value could be something or nothing.

The `Option` type was created the handle the "Billion Dollar Mistake" of
`null` values, invented by Tony Hare, which is famously known to plague
software development today.

## The `match` Control Flow Construct

An additional control flow construct that Rust contains, in addition the
common control flow constructs found in other languages, is the `match`
expression.

```struct
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### Matching with `Option<T>`

Combining the `Option` type with a `match` expression is the key abstraction
that Rust leans on the handle unknown data. This pattern is common in Rust
code bases.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(n) => Some(n + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

### Matches Are Exhaustive

The most important aspect of `match` to remember is: **the arms' patterns
must cover all possibilities**.

In the case that a section of a program does not need to deal with certain
variants of an enum, the `_` placeholder can be used.

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
```

## Concise Control Flow with `if let`

`if let` syntax combines `if` and `let` into a less verbose way to handle
values that match one pattern while ignoring the rest.

The following code:

```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {max}"),
    _ => (),
}
```

can be reduced to, using `if let`:

```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}
```

and further, to handle the case of `None`, an `else` expression can be used:

```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
} else {
    println!("No maximum configured");
}
```

