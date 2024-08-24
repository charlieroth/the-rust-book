fn main() {
    let word = String::from("This is a sentence");
    let length_of_first_word = length_of_first_word(&word);
    let first_word = first_word(&word);
    println!("Length of first word in '{word}' is {length_of_first_word}");
    println!("The first word in '{word}' is '{first_word}'");
}

fn length_of_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}
