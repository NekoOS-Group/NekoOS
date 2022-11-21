#![no_std]
#![no_main]

extern crate ulib;

#[no_mangle]
pub fn main() -> ! {
    unsafe {
        core::arch::asm!("sret");
    }
    panic!("FAIL: T.T\n");
}
