#![no_std]
#![no_main]

extern crate libc;
mod m;

use core::slice;

#[no_mangle]
pub extern "C" fn main(argc: isize, argv: *const *const u8) -> isize {
    let args = unsafe { slice::from_raw_parts(argv, argc as usize) };
    for i in 0..(argc as usize) {
        unsafe {
            libc::printf("arg #%d: %s\n\0".as_ptr() as *const _, i, args[i]);
        }
    }
    for i in 0..4 {
        libc_println!("#{} this is a {}", i, "book");
    }
    0
}
