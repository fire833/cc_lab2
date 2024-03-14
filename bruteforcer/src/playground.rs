use std::mem::{size_of_val, transmute};

use libc::{c_int, c_void};

pub struct Playground {
    raw_memory: *mut c_void,

    size: u32,
}

impl Playground {
    // Create a new memory region of a specific size so we can
    // run a bunch of crap inside and see if it produces correct output.
    unsafe fn new(size: u32) -> Self {
        let pointer: *mut c_void = std::ptr::null_mut();
        libc::mmap(
            pointer,
            size as usize,
            libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        );

        Self {
            raw_memory: pointer,
            size,
        }
    }

    // Provide raw bytes to copy over to executable memory, and run, return the output array
    fn run(&mut self, func: &[u8], permute_size: u32) -> Vec<u32> {
        let f: fn(*mut c_int, *mut c_int);
        let input: *mut c_int;
        let output: *mut c_int;

        // Clear out our memory region before placing some new crap there.
        unsafe {
            libc::memset(
                self.raw_memory,
                0,
                size_of_val(&(0 as u32)) * self.size as usize,
            );
            f = transmute(self.raw_memory);
            input = libc::calloc(permute_size as usize, size_of_val(&(0 as u32))) as *mut c_int;
            output = libc::calloc(permute_size as usize, size_of_val(&(0 as u32))) as *mut c_int;
        }

        f(input, output);

        vec![]
    }
}

impl Drop for Playground {
    fn drop(&mut self) {
        unsafe { libc::munmap(self.raw_memory, self.size as usize) };
    }
}
