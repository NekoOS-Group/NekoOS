use alloc::string::String;
use alloc::vec::Vec;
use alloc::sync::Arc;

use crate::schedule::task; 
use crate::mm;

pub enum State {
    Active,
    Blocked,
    Stopped{
        exit_code :      isize
    }
}

pub struct Stats {
    utime:               u64,
    stime:               u64,
    start_time:          u64,
    trap_count:          usize,
}

pub struct Infos {
    cwd:                 String,
    command:             String,
    name:                String
}

pub struct Process {
    pid:                 usize,
    state:               State,
    stats:               Stats,
    infos:               Infos,

    parent:              Option<Arc<Process>>,
    children:            Vec<Arc<Process>>,
    threads:             Vec<Arc<task::Thread>>,

    vm_space:            Arc<mm::VmSpace>,
    kernel_stack:        (),
}

impl Process {
    pub fn get_pid(&self) -> usize { self.pid }
    pub fn get_vm(&self) -> Arc<mm::VmSpace> { self.vm_space.clone() }

    pub fn new(parent: Option<Arc<Process>>, vm: Arc::<mm::VmSpace>, infos: Infos) -> Self {
        Self { 
            pid: task::pid_allocator::alloc().unwrap(), 
            state: State::Active, 
            stats: Stats { utime: 0, stime: 0, start_time: 0, trap_count: 0 }, 
            infos, 
            parent, 
            children: Vec::new(), 
            threads: Vec::new(), 
            vm_space: vm, 
            kernel_stack: () 
        }
    }

    pub fn new_kernel_proc() -> Arc<Self> {
        todo!()
    }

    pub fn new_elf() -> Arc<Self> {
        todo!()
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        super::pid_allocator::dealloc(self.pid);
    }
}