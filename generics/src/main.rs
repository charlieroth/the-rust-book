fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest: &i32 = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest: &char = &list[0];

    for character in list {
        if character > largest {
            largest = character;
        }
    }

    largest
}

// Since the function performs the binary comparison `>`, the generic type `T` must
// implement the `std::cmp::PartialOrd` trait
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
    let largest_i32 = largest_i32(&number_list);
    println!("The largest number is {largest_i32}");

    let character_list: Vec<char> = vec!['c', 'h', 'a', 'r', 'l', 'i', 'e'];
    let largest_char = largest_char(&character_list);
    println!("The largest character is {largest_char}");

    let character_list: Vec<char> = vec!['r', 'o', 't', 'h'];
    let largest_char = largest(&character_list);
    println!("The largest character is {largest_char}");
}
