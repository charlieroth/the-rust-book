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
}
