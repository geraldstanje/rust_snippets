fn main() {
    let mut x = 5;

    add_one(&mut x);
    println!("add one: {}" , x);
    
    add_two(&mut x);
    println!("add two: {}" , x);
}

fn add_one(num: &mut i32) { *num += 1; }

fn add_two(num: *mut i32) { unsafe { *num += 2; } }