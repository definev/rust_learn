// multiple producer, single consumer
use std::{sync::mpsc, thread, time::Duration};

#[allow(dead_code)]
fn message_passing_example() {
    // Transfering channel | Receiving channel
    let (tx, rx) = mpsc::channel();

    // Cloning new transfering channel it can transfering data to receiving schannel too
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

#[cfg(test)]
mod tests {
    use crate::message_passing_example::message_passing_example;

    #[test]
    fn message_passing_test() {
        message_passing_example();
    }
}
