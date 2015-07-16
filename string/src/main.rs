fn main() {
    let mystr1: &'static str = "This is a readonly string";
    
    for c in mystr1.chars().rev() {
        print!("{}", c);
    }   
    
    println!("");
    
    let mut mystr2 = String::new();
    for c in mystr1.chars() {
        mystr2.push(c);
    }
    
    println!("{}", mystr2);

    mystr2 = "Hello World".to_string();

    println!("{}", mystr2);

    let num: u32 = 12;

    mystr2 = num.to_string();

    println!("{}", mystr2);
}