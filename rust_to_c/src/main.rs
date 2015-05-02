extern crate libc;

extern {
    fn add(input1: libc::c_int, input2: libc::c_int) -> libc::c_int;
}

#[test]
fn it_works() {
    assert_eq!(3, unsafe { add(1, 2) });
}

fn main() {
    let input1 = 4;
    let input2 = 3;
    let output = unsafe { add(input1, input2) };
    println!("{} + {} = {}", input1, input2, output);
}
