use core::mem::size_of;
use std::mem::transmute;

use libc::{c_int, c_void};

use crate::optimize::ShiftMask;

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
    fn run(&self, func: &Vec<u8>, input: &Vec<u32>) -> Vec<u32> {
        let runner: extern "C" fn(*mut c_int, *mut c_int);
        let f_input: *mut c_int;
        let f_output: *mut c_int;

        // Clear out our memory region before placing some new crap there.
        println!("clearing/allocating memory now");
        unsafe {
            libc::memset(self.raw_memory, 0x00, self.size as usize);
            f_input = libc::calloc(input.len() as usize, size_of::<i32>()) as *mut c_int;
            f_output = libc::calloc(input.len() as usize, size_of::<i32>()) as *mut c_int;
        }

        // Placing inputs in memory.
        for (i, val) in input.iter().enumerate() {
            unsafe {
                *f_input.offset(i as isize) = *val as i32;
            }
        }

        let byte_ptr = self.raw_memory as *mut u8;
        // Placing program in memory
        for (i, byte) in func.iter().enumerate() {
            unsafe { *byte_ptr.offset(i as isize) = *byte }
        }

        // println!("transmuting function now: {:?}, {:?}", f_input, f_output);
        unsafe {
            runner = transmute(self.raw_memory);
        }

        // println!("running function now");
        runner(f_input, f_output);

        // Copy over output to vec.
        let mut out = vec![];

        for (i, _) in input.iter().enumerate() {
            let retval: i32;
            unsafe {
                retval = *f_output.offset(i as isize);
            }
            out.push(retval as u32);
        }

        unsafe {
            libc::free(f_input as *mut c_void);
            libc::free(f_output as *mut c_void);
        }

        out
    }

    pub fn run_is_correct(&self, func: &Vec<u8>, shift: &ShiftMask) -> bool {
        let permute_in = (1..(shift.len() + 1) as u32).collect::<Vec<u32>>();
        let res = self.run(func, &permute_in);
        let permute_out = shift.permute_array_by_mask(&permute_in);

        permute_out.eq(&res)
    }
}

impl Drop for Playground {
    fn drop(&mut self) {
        unsafe { libc::munmap(self.raw_memory, self.size as usize) };
    }
}
