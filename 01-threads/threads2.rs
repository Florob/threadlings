// threads2.rs
// Give the child thread a name
// Scroll down for hints

use std::thread;

fn child() -> Option<String> {
    let thread = thread::current();
    let thread_name = thread.name();
    println!("Hello from child(): thread name = {:?}", thread_name);
    thread_name.map(ToOwned::to_owned)
}

fn main() {
    println!(
        "Hello from main(): thread name = {:?}",
        thread::current().name()
    );

    let handle = thread::spawn(child);

    let thread_name = handle.join().unwrap();

    assert_eq!("child", thread_name.unwrap());
}
























// `std::thread::Builder` can be used to create threads with a name
