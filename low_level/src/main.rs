// from crates.io
extern crate mmap;
extern crate libc;

use std::thread;
use std::env;
mod fpga_awg;

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        let mut awg = fpga_awg::fpgaAwg::new();

        while (true) {
            awg.toggle(1);
            thread::sleep_ms(arg1.parse::<u32>().unwrap());
        }
    }
}