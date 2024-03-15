use core::mem::size_of;
use std::mem::transmute;

use libc::{c_int, c_void};

pub struct Playground {
    raw_memory: *mut c_void,

    size: u32,
}

impl Playground {
    // Create a new memory region of a specific size so we can
    // run a bunch of crap inside and see if it produces correct output.
    pub unsafe fn new(size: u32) -> Self {
        let empty: *mut c_void = std::ptr::null_mut();
        let pointer = libc::mmap(
            empty,
            size as usize,
            libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        );

        if pointer == 0x0 as *mut c_void {
            panic!("unable to allocate address space for program testing");
        }

        Self {
            raw_memory: pointer,
            size,
        }
    }

    // Provide raw bytes to copy over to executable memory, and run, return the output array
    pub fn run(&self, func: &Vec<u8>, permute_size: u32) -> Vec<u32> {
        let runner: extern "C" fn(*mut c_int, *mut c_int);
        let input: *mut c_int;
        let output: *mut c_int;

        let values = (1..permute_size).collect::<Vec<u32>>();

        // Clear out our memory region before placing some new crap there.
        println!("clearing/allocating memory now");
        unsafe {
            libc::memset(self.raw_memory, 0x00, self.size as usize);
            input = libc::calloc(permute_size as usize, size_of::<i32>()) as *mut c_int;
            output = libc::calloc(permute_size as usize, size_of::<i32>()) as *mut c_int;
        }

        // for (i, val) in values.iter().enumerate() {
        //     input.wrapping_add(i) = val;
        // }

        println!("transmuting function now: {:?}, {:?}", input, output);
        unsafe {
            runner = transmute(self.raw_memory);
        }

        println!("running function now");
        runner(input, output);

        unsafe {
            libc::free(input as *mut c_void);
            libc::free(output as *mut c_void);
        }

        vec![]
    }
}

impl Drop for Playground {
    fn drop(&mut self) {
        unsafe { libc::munmap(self.raw_memory, self.size as usize) };
    }
}
