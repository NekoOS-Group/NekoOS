#![no_std]
#![no_main]
extern crate ulib;

#[no_mangle]
pub fn main() -> isize {
    unsafe {
        #[allow(clippy::zero_ptr)]
        (0x0 as *mut u8).write_volatile(0);
    }
    panic!("FAIL: T.T\n");
}
