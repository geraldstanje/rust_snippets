use std::thread;

static NTHREADS: i32 = 10;

// This is the `main` thread
fn main() {
    let mut thread_vec: Vec<_> = Vec::new();
    
    for i in 0..NTHREADS {
        // Spawn up another thread
        let child = thread::spawn(move || {
            println!("this is thread number {}", i)
        });
        thread_vec.push(child);
    }
    
    for handle in thread_vec {
        // .unwrap() will propagate a panic from the child thread to the whole program
        handle.join().unwrap();
    }
}