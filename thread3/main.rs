use std::sync::{Arc, Mutex};
use std::thread;

// calculate pi multithreaded
// based on riemann integration

// This code is editable and runnable!
fn main() {
    const NUM_THREADS: u64 = 6;
    const NUM_STEPS: u64 = 100_000;
    const THREAD_STEPS: u64 = NUM_STEPS / NUM_THREADS;
    const STEP: f64 = 1.0 / NUM_STEPS as f64;

    let pi: Arc<Mutex<f64>> = Arc::new(Mutex::new(0.0f64));
    
    let guards: Vec<_> = (0..NUM_THREADS).map(|i| {
        let lower: u64 = THREAD_STEPS *i;
        let upper: u64 = THREAD_STEPS * (i+1);
        let pi_ref = pi.clone();

        thread::spawn(move || {
            for j in lower..upper {
                let x: f64 = (j as f64 + 0.5) * STEP;
                let mut lock = pi_ref.lock().unwrap();
                *lock += 4.0/(1.0 + x*x) * STEP;
            }
        })
    }).collect();

    for g in guards { g.join().unwrap(); }
    
    println!("Pi = {:.10}", *pi.lock().unwrap());
}