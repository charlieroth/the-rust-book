# Crates

A crate is a collection of Rust source code files.

There are *binary* crates, an executable, and *library* crates, contains
that is intended to be used in other programs and can't be executed
on its own.

`cargo` fetches the latest versions of everything that dependency needs
from the *registry*, a copy of data from [Crates.io](https://crates.io).  
Crates.io is where the Rust ecosystem post their open source Rust projects 
for others to use.
