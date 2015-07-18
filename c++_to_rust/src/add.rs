#![crate_type = "staticlib"]

#![feature(libc)]
extern crate libc;
use libc::{c_int};

#[no_mangle]
pub extern "C" fn add(a: c_int, b: c_int) -> c_int {
  return a + b;
}