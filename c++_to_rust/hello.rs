#![crate_type = "staticlib"]

#![feature(libc)]
extern crate libc;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn hello(name: *const libc::c_char) {
  let buf = unsafe { CStr::from_ptr(name).to_bytes() };
  let str_name = String::from_utf8(buf.to_vec()).unwrap();
  println!("Hello from Rust: {}!", str_name);
}