use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    let second: &i32 = &v[1];
    println!("The second element is {second}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("No third element"),
    }

    // Create a new hash map
    let mut scores = HashMap::new();

    // Add elements
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Get value of a key in a borrow safe way
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Team '{team_name}' score is {score}");

    // Iterate, in borrow-safe manner, of a hash map
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // A a key,value only if key is not present
    let _ = scores.entry(String::from("Green")).or_insert(30);

    println!("{scores:?}");
}
