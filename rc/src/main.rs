enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // When Rc::clone is used, it increases the reference count to `a` in this case
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    // When the `c`, `b` and `a` go out of scope, the reference count is decreased
    // thereby allowing the compiler/runtime to ensure no ownership is left unaccounted for
}

