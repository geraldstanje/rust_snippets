// from crates.io
extern crate mmap;
extern crate libc;

use std::thread;
use std::env;
mod fpga_hk;

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        let mut hk = fpga_awg::fpgaHk::new();

        while (true) {
            hk.toggle(1);
            thread::sleep_ms(arg1.parse::<u32>().unwrap());
        }
    }
}