use alloc::sync::Arc;
use crate::mm;
use crate::schedule::task;

pub struct Thread {
    tid: usize,

    proc: Arc<task::Process>,
    vm_space: Arc<mm::VmSpace>
}

impl Thread {
    pub fn get_tid(&self) -> usize { self.tid }

    pub fn new(proc: Arc<task::Process>) -> Self {
        Self { 
            tid: task::tid_allocator::alloc().unwrap(),
            vm_space: proc. get_vm(),
            proc, 
        }
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        task::tid_allocator::dealloc(self.get_tid())
    }
}