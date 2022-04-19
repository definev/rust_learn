use std::thread;
use std::time::Duration;

/// *Thread* methods
/// - `thread::spawn` for spawning new thread, this is implement by operating systems
/// - `thread::sleep` delaying current thread in closure
/// - `thread::spawn` return a `JoinHandle`
/// - `JoinHandle` with `join` method waiting all thread in spawn closure done their work.
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    let res = handle.join();
    match res {
        Ok(data) => println!("{data:?}"),
        _ => {}
    }

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
