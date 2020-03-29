#![no_std]
#![no_main]

extern crate libc;
mod m;

#[no_mangle]
pub extern "C" fn main(argc: isize, argv: *const *const u8) -> isize {
    for i in 0..(argc as usize) {
        unsafe {
            libc::printf("arg #%d: %s\n\0".as_ptr() as *const _, i, *argv.offset(i as isize));
        }
    }
    
    for i in 0..4 {
        libc_println!("#{} this is a {}", i, "book");
    }
    0
}
