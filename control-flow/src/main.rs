fn main() {
    // === If/Else If/Else ===
    let number = 3;
    if number % 5 == 0 {
        println!("number is divisible by 5");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 5 or 3");
    }

    let condition = true;
    let other_number = if condition { 5 } else { 3 };
    println!("The value of other number is: {}", other_number);

    // === Repetition with Loops
    let mut i = 0;
    let hello_count: i32 = loop {
        if i == 5 {
            break i;
        }

        println!("hello!");
        i += 1;
    };
    println!("Said 'hello!' {} times", hello_count);

    // === Loop labels ===
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // === While loops ===
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!");

    // === Loop Through a Collection with `for` ===
    let a = [10, 20, 30, 40, 50];
    for elem in a {
        println!("The value is: {elem}");
    }
}
