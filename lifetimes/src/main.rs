// Rust requires a lifetime annotation here because
// the `if/else` statement creates a potential for
// two lifetimes of references. By specifying the
// same lifetime for both, the borrow checker knows
// how to proceed when the function is used
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime annotations in Struct Definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn main() {
    let x = String::from("abcd");
    let y = "xyz";
    let result = longest(x.as_str(), y);
    println!("The longest string is {result}");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("level: {}", i.level())
}
