fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

    let v2 = vec![1, 2, 3];
    let v3: Vec<i32> = v2.iter().map(|x| x + 1).collect();
    println!("v3: {:?}", v3);
}
