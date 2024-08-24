#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

struct Color(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("charlieroth"),
        email: String::from("charlie@email.com"),
        sign_in_count: 1,
    };

    // Get a struct with dot notation
    println!("User's email is {}", user1.email);

    // Update a field on the struct
    user1.email = String::from("charlie_updated@email.com");
    println!("User's email is now {}", user1.email);

    // Create a new instance from other struct instances with struct
    // update syntax
    let user2 = User {
        email: String::from("charlie2@email.com"),
        ..user1
    };
    println!("User 2 email is {}", user2.email);

    let black = Color(0, 0, 0);
    println!(
        "The color black in rgb notation is ({}, {}, {})",
        black.0, black.1, black.2
    );

    let r1 = Rectangle { width: 5, height: 5 };
    println!("The area of the rectangle is {}", r1.area());

    let r2 = Rectangle { width: 4, height: 4 };
    println!("Can r1 hold r2? {}", r1.can_hold(&r2));
}
