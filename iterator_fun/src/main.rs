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
    
    for num in nums.iter() {
        println!("{}", num);
    }
    
    for num in nums.into_iter() {
        println!("{}", num);
    }
    
    //for num in nums.iter() {
    //    println!("{}", num);
    //}
}