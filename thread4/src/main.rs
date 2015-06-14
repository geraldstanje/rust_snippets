use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

// This code is editable and runnable!
fn main() {
    let mut thread_vec: Vec<_> = Vec::new();
     
    let mut movie_reviews: HashMap<u8, &str> = HashMap::new();
    movie_reviews.insert(0, "Terminator");
    movie_reviews.insert(1, "Lord of the Rings");
    movie_reviews.insert(2, "Rambo");
    movie_reviews.insert(3, "Die Hard");
    
    // this is a atomic refer counter
    // if the ref counter gets 0, the hashmap can get destroyed
    let arc = Arc::new(movie_reviews);
    
    for i in 0..4 {
        let arc1 = arc.clone();
        
        let child = thread::spawn(move || {
            println!("{}", arc1[&(i as u8)]);
        });
        
        thread_vec.push(child);
    }
    
    for handle in thread_vec {
        handle.join().unwrap();
    }
}
