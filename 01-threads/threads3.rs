// threads3.rs
// Make this compile
// Scroll down for hints

use std::thread;

const HEIGHT: usize = 100;
const WIDTH: usize = 200;
const THREADS: usize = 4;

fn calculate_mandlebrot_slice(start: usize, end: usize) -> Vec<[bool; WIDTH]> {
    let mut slice = Vec::with_capacity(end - start);
    for y in start..end {
        let mut row = [true; WIDTH];
        for (x, cell) in row.iter_mut().enumerate() {
            let c = (
                -2.0 + 2.5 * x as f32 / WIDTH as f32,
                -1.0 + 2.0 * y as f32 / HEIGHT as f32,
            );
            let mut z = (0f32, 0f32);
            for _ in 0..65_000 {
                z = (z.0 * z.0 - z.1 * z.1 + c.0, z.0 * z.1 + z.1 * z.0 + c.1);
                if z.0 * z.0 + z.1 * z.1 > 4.0 {
                    *cell = false;
                    break;
                }
            }
        }
        slice.push(row);
    }
    slice
}

fn main() {
    let threads: Vec<_> = (0..THREADS)
        .map(|i| {
            thread::spawn(|| {
                calculate_mandlebrot_slice(i * HEIGHT / THREADS, (i + 1) * HEIGHT / THREADS)
            })
        })
        .collect();

    for thread in threads {
        let slice = ...;
        for row in slice {
            for &cell in row.iter() {
                if cell {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }
}

























// Variables referenced from a thread need to have `'static` lifetime, so the thread
// can't outlive them. Instead we can make the thread own the data:
// <https://doc.rust-lang.org/book/ch16-01-threads.html#using-move-closures-with-threads>
// Remember to use `join()` to get data returned from threads as in previous exercises.
