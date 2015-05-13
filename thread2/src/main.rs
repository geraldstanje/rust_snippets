use std::thread;

fn main() {
  let mut thread_vec: Vec<_> = Vec::new();
  let nums = [1, 2];
  let noms = ["gerald", "chriss", "martin", "mike"];
  
  let mut odds = nums.iter().map(|&x| x * 2 - 1);
  
  for num in odds {
    let child = thread::spawn(move || {
      println!("{} says hello from a thread!", noms[num]);
    });
    thread_vec.push(child);
  }

  for handle in thread_vec {
        // .unwrap() will propagate a panic from the child thread to the whole program
        // .unwrap() just panics on None/Err(e)
        // And returns the value from Some(x)/Ok(x)

        handle.join().unwrap();
    }
}