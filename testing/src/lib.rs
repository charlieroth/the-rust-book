pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be >= 1");
        }

        if value > 100 {
            panic!("Guess value must be <= 100");
        }

        Guess { value }
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    String::from("hello")
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    // Bring all functions and types into scope of the `tests` module
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn use_result() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail")
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };
        let smaller = Rectangle {
            width: 4,
            height: 8,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_bigger() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };
        let smaller = Rectangle {
            width: 4,
            height: 8,
        };

        assert!(!smaller.can_hold(&larger));
    }

    // #[test]
    // fn greeting_contains_name() {
    //     let result = greeting("Charlie");
    //     // Assertion with custom error message
    //     assert!(
    //         result.contains("Charlie"),
    //         "Greeting did not contain name, value was {result}"
    //     );
    // }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(101);
    }

    #[test]
    #[should_panic(expected = "Guess value must be >= 1")]
    fn less_than_1() {
        Guess::new(-10);
    }
}
