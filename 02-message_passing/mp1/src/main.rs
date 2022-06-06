// mp1
// Make this compile
// Scroll down for hints

use rand::Rng;
use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

mod averager;
use averager::Averager;

const THROWS: u32 = 10_000_000;
const THREADS: u8 = 4;
const MAX_RUNTIME: Duration = Duration::from_secs(30);

/// Throw `count` darts at a 2x2 square with a unit circle on it
/// Returns the number of darts that landed inside the circle
fn throw_darts(mut rng: impl Rng, count: u32) -> u32 {
    let mut inside = 0;
    for _ in 0..count {
        let x = rng.gen_range(-1f64, 1f64);
        let y = rng.gen_range(-1f64, 1f64);
        if x * x + y * y <= 1f64 {
            inside += 1;
        }
    }
    inside
}

fn throw_thread(sender: mpsc::Sender<f64>) {
    let mut rng = rand::thread_rng();

    loop {
        let inside = throw_darts(&mut rng, THROWS);
        if sender
            .send(4.0 * f64::from(inside) / f64::from(THROWS))
            .is_err()
        {
            // Receiver disappeared, stop computing
            return;
        }
    }
}

fn main() {
    let mut pi_averager = Averager::new();

    let (tx, rx) = mpsc::channel();

    for _ in 0..THREADS {
        thread::spawn(|| throw_thread(tx));
    }

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let start = Instant::now();
    for pi in rx {
        pi_averager.update(pi);

        // Print out the current estimate for PI, always overwriting the same line
        write!(stdout, "\r\u{03c0} \u{2248} {:.10}", pi_averager.average()).unwrap();
        stdout.flush().unwrap();

        // Ran for MAX_RUNTIME, finish line and quit
        if start.elapsed() >= MAX_RUNTIME {
            println!();
            return;
        }
    }
}
























//  Have a closer look at the `std::sync::mpsc::channel` type.
//  Paricularly look into the meaning of MPSC.
