use std::ptr;
use std::fs;
use std::io::{Write, SeekFrom, Seek};
use std::os::unix::prelude::AsRawFd;
use mmap::{MemoryMap, MapOption};
use libc;
use std::f32;
use std::default::Default;

const HK_BASE_ADDR: isize = 0x000000;

struct hk_reg {
  id: u32,
  dna_low: u32,
  dna_high: u32,
  reserved1: u32,
  exp_dir_p: u32,
  exp_dir_n: u32,
  exp_out_p: u32,
  exp_out_n: u32,
  exp_in_p: u32,
  exp_in_n: u32,
  reserved2: u32,
  reserved3: u32,
  led_ctrl: u32, // 0x30
}

pub struct fpgaHk {
  mmap: MemoryMap,
  hkReg: *mut hk_reg,
}

impl fpgaHk {
  pub fn new() -> fpgaHk {  
    let size: usize = 0x400000;

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

    let hkReg: *mut hk_reg;

    unsafe {
        hkReg = data.offset(HK_BASE_ADDR) as *mut hk_reg;
    }

    fpgaHk {mmap: mmap, hkReg: hkReg}
  }

  pub fn toggle(&mut self, led_pin: u32) {
      unsafe {
          (*self.hkReg).led_ctrl ^= 1 << led_pin;
      }
  }

  pub fn set_exp_p_dir(&mut self, dir: u32) {
      unsafe {
          (*self.hkReg).exp_dir_p = dir;
      }
  }

  pub fn toggle_exp_p_gpo(&mut self, pin: u32) {
      unsafe {
          (*self.hkReg).exp_out_p ^= 1 << pin;
      }
  }

  pub fn clear_exp_p_gpo(&mut self, pin: u32) {
      unsafe {
          (*self.hkReg).exp_out_p = !(1 << pin);
      }
  }

  pub fn set_exp_p_gpo(&mut self, pin: u32) {
      unsafe {
          (*self.hkReg).exp_out_p |= 1 << pin;
      }
  }
}