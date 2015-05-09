fn main() {
    let mut nums = vec![1, 2, 3];

    for num in nums.iter_mut() {
        *num *= 2;
    }
    
    for num in &mut nums {
        *num *= 2;
    }
    
    for num in &nums {
        println!("{}", num);
    }
    
    // x.iter() ... iter() borrows x and returns an iterator to it
    // and once the iterator is dropped you can use x again
    for num in nums.iter() {
        println!("{}", num);
    }
    
    // x.into_iter() ... it borrows x and returns borrowed references into x
    // with into_iter you can never use x again because it moved away
    for num in nums.into_iter() {
        println!("{}", num);
    }
    
    //for num in nums.iter() {
    //    println!("{}", num);
    //}
}