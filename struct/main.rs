struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn new(w: i32, h: i32) -> Rectangle {
        Rectangle { width: w, height: h }
    }
    
    fn area(&self) -> f32 {
        return (self.width * self.height) as f32;
    } 
}


fn main() {
    let r = Rectangle::new(3, 2);
    
    println!("{}", r.area());
}