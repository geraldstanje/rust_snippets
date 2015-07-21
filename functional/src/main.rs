struct MyStruct {
    a: u32,
    b: u32,
}
    
fn main() {
    let mut vec : Vec<MyStruct> = Vec::new();
    
    vec.push(MyStruct{a: 1, b: 2});
    vec.push(MyStruct{a: 2, b: 2});
    
    for e in vec.into_iter().filter(|x| x.a > 1) {
        println!("{}", e.b);
    }
}