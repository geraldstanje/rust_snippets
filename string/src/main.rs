pub fn sort(word: &'static str) -> String {
    let mut chars: Vec<char> = word.chars().collect();     
    chars.sort();
    return chars.into_iter().collect();
}

fn extract_numbers(s: &str) -> Vec<String> {
    let mut num = String::new();
    let mut m = vec![];
    
    for c in s.chars() {
        if c.is_numeric() {
            num.push(c);
        }
        else {
            if !num.is_empty() {
                m.push(num.clone());
                num.clear();
            }
        }
    }
    
    if !num.is_empty() {
        m.push(num);
    }
    
    return m;
}

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

    let str = sort("abzerfdsgyiof");
    print!("{}", str);
    
    let mystr3 = "x12y7".to_string();
    let m: Vec<String> = extract_numbers(&mystr3); // -> ( "12", "7" )
    
    println!("");
    
    for x in m.iter() {
        println!("{}", x);
    }
}