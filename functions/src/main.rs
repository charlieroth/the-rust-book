fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn give_me_five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');

    // === Statements ===
    let y = 6;
    println!("Value of y is: {y}");

    // === Expressions ===
    let y = {
        let x = 3;
        x + 1
    };
    println!("Value of y is: {y}");

    let x = give_me_five();
    println!("Value of x is: {x}");

    let z = plus_one(5);
    println!("Value of z is: {z}");
}
