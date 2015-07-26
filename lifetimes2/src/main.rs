use std::collections::HashMap;
use std::collections::hash_map::Values;

// the contents of the method called values doesn't matter for the borrow checker, just its signature
struct Foo {
    pub map: HashMap<i32, String>,
    pub bar: Bar,
}

struct Bar {
    pub count: usize,
}

impl Bar {
    pub fn update<'a>(&mut self, values: Values<'a, i32, String>) {
        self.count = values.len();
    }
}

impl Foo {
    pub fn new() -> Foo {
        Foo{
            map: HashMap::new(),
            bar: Bar{count: 0},
        }
    }
    
    #[allow(dead_code)]
    pub fn values<'a>(&'a self) -> Values<'a, i32, String> {
        self.map.values()
    }
    
    pub fn update(&mut self) {
        // OK
        // self.map.values() only borrows self.map
        self.bar.update(self.map.values());
        
        // Not OK
        // `self.map.values()` but not call a function because self.values() borrows the whole `self`
        // self.bar.update(self.values());
    }
}

fn main() {
    let mut foo = Foo::new();
    println!("Got count: {}", foo.bar.count);
    
    {
        foo.map.insert(5, "Hello".into());
        foo.update();
        println!("Got count: {}", foo.bar.count);
    }
}