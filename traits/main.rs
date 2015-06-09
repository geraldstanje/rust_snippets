trait Polygon {
    // Static method signature; `Self` refers to the implementor type
    fn new(width: i32, height: i32) -> Self;
    
    fn area(&self) -> f32;
}

struct Rectangle { width: i32, height: i32 }

impl Polygon for Rectangle {
    fn new(w: i32, h: i32) -> Rectangle {
        Rectangle { width: w, height: h }
    }

    fn area(&self) -> f32 {
        return (self.width * self.height) as f32;
    }
}

struct Triangle { width: i32, height: i32 }

impl Polygon for Triangle {
    fn new(w: i32, h: i32) -> Triangle {
        Triangle { width: w, height: h }
    }

    fn area(&self) -> f32 {
        return self.width as f32 * self.height as f32 / 2.0;
    }
}

fn main() {
    let r: Rectangle = Polygon::new(2, 3);
    println!("{}", r.area());
    
    let t: Triangle = Triangle::new(2, 3);
    println!("{}", t.area());
}
