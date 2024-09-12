use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        let msgs = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for msg in msgs {
            tx.send(msg).expect("Transmitter failed to send message");
            thread::sleep(Duration::from_millis(500));
        }
    });

    for msg in rx {
        println!("Received: {msg}");
    }
}
