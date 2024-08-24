#[allow(unused_variables)]

fn main() {
    // ----- Scalar Types -----
    // === Numerical Types ===
    let a = 1; // default data type is `i32`
    let b: i32 = 10; // expclit data type annotation
    let c = 2.0; // default data type is `f64`
    let d: f32 = 3.0;
    // === Numeric Operations ===
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    let remainder = 43 % 5;
    // === Booleans ===
    let e = true;
    let f: bool = false;
    // === Characters ===
    let g = 'z';
    let h: char = 'Z';
    let heart_eyed_cat = '😻';

    // ----- Compound Types -----
    // === Tuple ===
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (tup_x, tup_y, tup_z) = tup;
    println!("The value of y is: {tup_y}");
    println!("The value of x is: {}", tup.0);
    // === Arrays ===
    let i = [1, 2, 3, 4, 5];
    let i_2 = [3; 5]; // same as [3, 3, 3, 3, 3]
    let months = ["Jan", "Feb", "March", "April"];
}
