pub fn backtrace() {
    use crate::config::{stext, etext, skernel};

    let mut fp = crate::arch::register::get_fp();
    let mut pc = crate::arch::register::get_ra();
    let mut deep = 0;

    info!("=== begin Neko stack trace ===");

    while pc >= stext as usize
       && pc <= etext as usize
    {   
        info!(
            "  {:02} PC: {:#018x} FP: {:#018x}",
            deep,
            pc - core::mem::size_of::<usize>(),
            fp
        );
        if fp < skernel as usize {
            break;
        }
        unsafe { 
            fp = *(fp as *const usize).offset(-2);
            pc = *(fp as *const usize).offset(-1);
        }
        deep += 1;
    }

    info!("=== end Neko stack trace ===");
}