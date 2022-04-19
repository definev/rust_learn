use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

#[allow(dead_code)]
fn spawnding_thread() -> JoinHandle<()> {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Sending from thread {i}!!!");
            thread::sleep(Duration::from_millis(1));
        }
    })
}

#[allow(dead_code)]
fn main_thread() {
    for i in 1..5 {
        println!("Sending from main {i}!!!");
    }
}

#[allow(dead_code)]
fn blocking_thread_example() {
    spawnding_thread();
    main_thread();
}

#[allow(dead_code)]
fn non_blocking_thread_example() {
    let handle = spawnding_thread();
    handle.join().unwrap();
    main_thread();
}

#[cfg(test)]
mod tests {
    use crate::thread_example::*;

    #[test]
    fn thread_test() {
        blocking_thread_example();
        println!();
        non_blocking_thread_example();
    }
}
