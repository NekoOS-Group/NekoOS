#![no_std]

mod lang;
pub mod syscall;

fn bss_init() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(
        |a| { unsafe { (a as *mut u8).write_volatile(0) } } 
    );
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    bss_init();
    syscall::exit(0);
}
