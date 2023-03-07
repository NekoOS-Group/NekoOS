use alloc::sync::Arc;
use crate::mm;
use crate::mm::page_table::PageTable;
use crate::task;
use crate::trap;

pub struct Thread {
    tid: usize,

    proc: Arc<task::Process>,
    vm_space: Arc<spin::Mutex<mm::VmSpace>>,

    context: Option<trap::Context>
}

impl Thread {
    pub fn get_tid(&self) -> usize { self.tid }

    pub fn new(proc: Arc<task::Process>) -> Self {
        Self { 
            tid: task::tid_allocator::alloc().unwrap(),
            vm_space: proc.get_vm(),
            proc, 
            context: None
        }
    }

    pub fn park(&mut self, context: trap::Context) {
        self.context = Some(context)
    }

    pub fn prepare(&mut self) -> *mut trap::Context {
        self.vm_space.lock().get_page_table().activate();
        let context = self.context.take().unwrap();
        task::current_processor().kernel_stack.push_context(context)
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        task::tid_allocator::dealloc(self.get_tid())
    }
}