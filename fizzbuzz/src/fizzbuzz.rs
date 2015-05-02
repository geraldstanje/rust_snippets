fn main() {
    for i in 1..101 {
        let value = i.to_string();
        println!("{}", match (i % 3, i % 5) {
                         (0,0) => "FizzBuzz",
                         (0,_) => "Fizz",
                         (_,0) => "Buzz",
                          _ => value.as_ref(),
                       });
    }
}