use std::ptr;
use std::fs;
use std::io::{Write, SeekFrom, Seek};
use std::os::unix::prelude::AsRawFd;
use mmap::{MemoryMap, MapOption};
use std::thread;
use std::env;

// from crates.io
extern crate mmap;
extern crate libc;

fn toggle(data: *mut u8, led_pin: u32) {
    unsafe {
        *(data.offset(0x30) as *mut u32) ^= 1 << led_pin;
    }
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        let size: usize = 0x1000;

        let mut f = fs::OpenOptions::new().read(true)
                                          .write(true)
                                          .open("/dev/uio0")
                                          .unwrap();

        let mmap_opts = &[
            // Then make the mapping *public* so it is written back to the file
            MapOption::MapNonStandardFlags(libc::MAP_SHARED),
            MapOption::MapReadable,
            MapOption::MapWritable,
            MapOption::MapFd(f.as_raw_fd()),
        ];

        let mmap = MemoryMap::new(size, mmap_opts).unwrap();

        let data = mmap.data();

        if data.is_null() {
            panic!("Could not access data from memory mapped file")
        }
        else {
            println!("successful data access to memory mapped file");
        }

        while (true) {
            toggle(data, 0);
            thread::sleep_ms(arg1.parse::<u32>().unwrap());
        }
    }
}
