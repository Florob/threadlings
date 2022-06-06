// sm2.rs
// Make this compile, without using a Mutex
// Scroll down for hints

use std::sync::Arc;
use std::thread;

const NUM_THREADS: u32 = 4;
const INCREMENTS: u32 = 1_000_000;

fn main() {
    let counter = Arc::new(0u32);

    let handles: Vec<_> = (0..NUM_THREADS)
        .map(|_| {
            let thread_counter = counter.clone();
            thread::spawn(move || {
                for _ in 0..INCREMENTS {
                    *thread_counter += 1;
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    assert_eq!(NUM_THREADS * INCREMENTS, *counter);
    println!("Success!");
}
























// The types in the `std::sync::atomic` module provide shared mutable access for some types.
