use alloc::string::String;
use alloc::vec::Vec;
use alloc::sync::Arc;

use crate::task; 
use crate::mm;

pub enum State {
    Uninited,
    Active,
    Blocked,
    Exited{
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

    vm_space:            Arc<spin::Mutex<mm::VmSpace>>,
}

impl Process {
    pub fn get_pid(&self) -> usize { self.pid }
    pub fn get_vm(&self) -> Arc<spin::Mutex<mm::VmSpace>> { self.vm_space.clone() }

    pub fn new(parent: Option<Arc<Process>>, vm: Arc::<spin::Mutex<mm::VmSpace>>, infos: Infos) -> Self {
        Self { 
            pid: task::pid_allocator::alloc().unwrap(), 
            state: State::Uninited, 
            stats: Stats { utime: 0, stime: 0, start_time: 0, trap_count: 0 }, 
            infos, 
            parent, 
            children: Vec::new(), 
            threads: Vec::new(), 
            vm_space: vm,
        }
    }

    pub fn new_kernel_proc(name: &str) -> Arc<Self> {
        todo!()
    }

    pub fn from_elf() -> Arc<Self> {
        todo!()
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        super::pid_allocator::dealloc(self.pid);
    }
}