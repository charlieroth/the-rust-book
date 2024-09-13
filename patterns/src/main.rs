enum Greeting {
    Hello { id: i32 },
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

struct Point {
    x: i32,
    y: i32,
}

// Function parameters can also be patterns
fn print_coordinates(&(x, y, z): &(i32, i32, i32)) {
    println!("Treasure located at coordinates: ({x},{y},{z})");
}

fn main() {
    // Conditional `if let` expressions
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // `while let` Conditional Loops
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("Popped: {top}");
    }

    // `for` Loops
    let v = vec![1, 2, 3, 4];
    for (index, value) in v.iter().enumerate() {
        println!("v[{index}]={value}");
    }

    // "Unpack" Tuples with `let` Statements
    let coorindates = (1, 2, 3);
    let (x, y, z) = coorindates;
    println!("Treasure located at coordinates: ({x},{y},{z})");
    print_coordinates(&coorindates);

    // Multiple Patterns
    let x = 1;
    match x {
        1 | 2 => println!("One or two"),
        3 => println!("Three"),
        4..=7 => println!("Between 4 and 7"),
        _ => println!("Anything"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("Early ASCII letter"),
        'k'..='z' => println!("Late ASCII letter"),
        _ => println!("Something else"),
    }

    // Destructuring Structures
    let p = Point { x: 0, y: 8 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(8, b);

    // Destructuring Enums
    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        // Destructuring Nested Structures & Enums
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to rgb({r},{g},{b})");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hsv({h},{s},{v})");
        }
    }

    // Extra conditionals with Match Guards
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    // `@` Bindings
    let greeting = Greeting::Hello { id: 5 };
    match greeting {
        Greeting::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Greeting::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Greeting::Hello { id } => println!("Found some other id: {id}"),
    }
}
