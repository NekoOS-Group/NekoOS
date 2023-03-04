use core::mem::size_of;

use crate::config;
use crate::trap::Context;

pub struct KernelStack {
    stack: &'static [u8; config::KERNEL_STACK_SIZE]
}

impl KernelStack {
    fn push_context(&mut self, context: Context) -> *mut Context {
        let pt = self.stack.as_ptr() as usize + size_of::<Self>() - size_of::<Context>();
        let ret = pt as *mut Context;

        unsafe { *ret = context } 

        ret
    }
}