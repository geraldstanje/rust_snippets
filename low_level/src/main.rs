// useful for debugging:
// strace -fo logfile /path/to/exe 
// strace -fe mmap /path/to/executable

use std::ptr;
use std::fs;
use std::io::{Write, SeekFrom, Seek};
use std::os::unix::prelude::AsRawFd;
use mmap::{MemoryMap, MapOption};
use std::thread;

// from crates.io
extern crate mmap;
extern crate libc;

fn toggle(data: *mut u8, led_pin: u32) {
    unsafe {
        *(data.offset(0x30) as *mut u32) ^= 1 << led_pin;
    }
}

fn main() {
    let size: usize = 0x7FFFFF; // i want to access the memory from 0x40000000 to 0x407FFFFF

    let mut f = fs::OpenOptions::new().read(true)
                                      .write(true)
                                      .open("/dev/mem")
                                      .unwrap();

    let mmap_opts = &[
        // Then make the mapping *public* so it is written back to the file
        MapOption::MapNonStandardFlags(libc::MAP_SHARED), //consts::os::posix88::MAP_SHARED),
        MapOption::MapNonStandardFlags(libc::consts::os::extra::O_SYNC),
        MapOption::MapReadable,
        MapOption::MapWritable,
        MapOption::MapOffset(0x40000000),
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
        toggle(data, 3);
        thread::sleep_ms(200);
    }
}