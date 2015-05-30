fn create1() -> Vec<&'static str> {
    let mut m = vec![];
    for n in 0..10 {
        m.push("hello");
    }
    m.clone()
}

fn create2(t: &str) -> Vec<&str> {
    let mut m = vec![];
    for n in 0..10 {
        m.push(t);
    }
    m.clone()
}

fn create3(t: &str) -> Vec<&str> {
    vec![t; 10]
}

fn main() {
    let mut u = create1();

    for n in u.iter() {
        println!("{}", n);
    }
    
    let mut v = create2("hello");

    for n in v.iter() {
        println!("{}", n);
    }

    let w = create3("hello");
    for n in &w {
        println!("{}", n);
    }
}