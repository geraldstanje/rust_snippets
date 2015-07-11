use num::traits::One;
use std::ops::Add;

extern crate num;

struct TwoNums<T> {
  a: T,
  b: T
}

// You implement function for struct TwuNums which accept type T with following traits: Add and One
impl<T> TwoNums<T> where T: Add<Output=T> {
  fn new(a: T, b: T) -> TwoNums<T> {
    TwoNums {a: a, b: b}
  }
}

// inside your funcion which accepts type your type T Rust expects that you pass to add() the variable 
// with the same Type
impl<T> TwoNums<T> where T: Add<Output=T> + One {
  fn succ(self) -> TwoNums<T> {
    let a = self.a.add(One::one());
    let b = self.b.add(One::one());
    TwoNums::new(a, b)
  }
}

fn main() {
  let num: TwoNums<f64> = TwoNums::new(1.0, 2.0);
  let snum: TwoNums<f64> = num.succ();
  println!("{}", snum.a);
  println!("{}", snum.b);
}
