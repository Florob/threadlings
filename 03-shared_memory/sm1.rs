// sm1.rs
// This approximates Pi by calculating the Riemann integral:
// $\pi = 4 * \arctan(1) = \int \limits_0^1 4/(1+x^2) dx$
//
// Make this compile
// Scroll down for hints
use std::thread;

const NUM_THREADS: u64 = 4;
const NUM_STEPS: u64 = 100_000;

fn main() {
    const THREAD_STEPS: u64 = NUM_STEPS / NUM_THREADS;
    const STEP: f64 = 1.0 / NUM_STEPS as f64;

    let pi = 0f64;

    let handles: Vec<_> = (0..NUM_THREADS)
        .map(|i| {
            let lower: u64 = THREAD_STEPS * i;
            let upper: u64 = THREAD_STEPS * (i + 1);
            thread::spawn(|| {
                for j in lower..upper {
                    let x: f64 = (j as f64 + 0.5) * STEP;
                    pi += 4.0 / (1.0 + x * x) * STEP;
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    println!("Pi = {:.10}", pi);
}
























// `std::sync::Mutex` can be used to allow threads mutually exclusive write access
// to otherwise immutably shared data.
//
// `std::sync::Arc` is an Atomic Reference Counted pointer which can be used for
// safe, shared access to **immutable** data.
//
// More information can be found in the book:
// https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html
