use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));

        match tx.send(String::from("hi")) {
            Ok(_) => println!("Transmitter sent message"),
            Err(e) => println!("Transmitter failed to send message {e}"),
        };
    });

    match rx.recv() {
        Ok(msg) => println!("Received: {msg}"),
        Err(e) => println!("Receiver failed to receive a message: {e}"),
    };
}
