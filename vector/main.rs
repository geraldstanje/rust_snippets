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

fn main() {
    let mut u = create1();

    for n in u.iter() {
        println!("{}", n);
    }
    
    let mut v = create2("hello");

    for n in v.iter() {
        println!("{}", n);
    }
}