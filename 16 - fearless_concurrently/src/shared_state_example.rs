use std::{sync::{Mutex, Arc}, thread};

#[allow(dead_code)]
fn shared_state_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()
    }

    println!("m = {:?}", counter.lock().unwrap());
}

#[cfg(test)]
mod tests {
    use super::shared_state_example;

    #[test]
    fn shared_state_example_test() {
        shared_state_example();
    }
}
