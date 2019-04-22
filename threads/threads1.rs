// threads1.rs
// Spawn the child function on a different thread, reading back the result
// Scroll down for hints

use std::thread;

fn child() -> thread::ThreadId {
    let thread_id = thread::current().id();
    println!("Hello from child(): ThreadId = {:?}", thread_id);
    thread_id
}

fn main() {
    let my_thread_id = thread::current().id();
    println!("Hello from main(): ThreadId = {:?}", my_thread_id);

    let other_thread_id = child();

    // Make sure main() and child() ran on different threads
    assert_ne!(my_thread_id, other_thread_id);
}
























// `std::thread::spawn` is used to spawn new threads.
// Also have a look at the `join()` function associated with the returned
// `std::thread::JoinHandle`.
