use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1))
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(1))
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3];
    // Use `move` to force closure to take ownership of the values
    // it's using rather than allowing Rust to infer that is should
    // borrow the values
    let handle = thread::spawn(move || {
        println!("A vector {v:?}");
    });

    handle.join().unwrap();
}
