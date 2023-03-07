use crate::mm;
use crate::task;

pub struct Processor {
    pub kernel_stack: mm::KernelStack,
    pub current_thread: task::ThreadRef
}