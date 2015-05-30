const n: usize = (16*1024);

struct foo {
    chaMem: *mut [u32; n],
}

impl foo {
    pub fn new() -> foo {
        let mem: *mut [u32; n] = unsafe { ::std::mem::transmute(Box::new([1; n])) };
        foo {chaMem: mem}
    }
    
    pub fn get_sig_ptr(&mut self, cha_signal: *mut *mut u32) {
        unsafe {
            *cha_signal = self.chaMem as *mut _;
        }
    }
    
    pub fn get_sig_ptr2(&mut self) -> *mut u32 {
        return self.chaMem as *mut _;
    }
    
    pub fn get_sig_ptr3(&mut self) -> *mut [u32; n] {
        return self.chaMem;
    }
    
    pub fn get_sig_ptr4(&mut self) -> &mut [u32; n] {
        unsafe {
            return &mut *(self.chaMem);
        }
    }
}

// This code is editable and runnable!
fn main() {
    let mut f = foo::new();
    
    let mut cha_signal1: *mut u32 = ::std::ptr::null_mut();
    f.get_sig_ptr(&mut cha_signal1);
    
    for i in 1..n {
        println!("{}", unsafe { *cha_signal1.offset(i as isize) }); // (*cha_signal)[i] 
    }
    
    let mut cha_signal2: *mut u32 = f.get_sig_ptr2();
    
    for i in 1..n {
        println!("{}", unsafe { *cha_signal2.offset(i as isize) });
    }
    
    let mut cha_signal3: *mut [u32; n] = f.get_sig_ptr3();
    
    for i in 1..n {
        println!("{}", unsafe { (*cha_signal3)[i] });
    }
    
    let mut cha_signal4: &mut [u32; n] = f.get_sig_ptr4();
    
    for i in 1..n {
        println!("{}", cha_signal4[i]);
    }
}