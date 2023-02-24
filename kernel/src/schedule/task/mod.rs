use alloc::collections::BTreeMap;
use alloc::sync::Arc;

mod process;
mod pid_allocator;
mod process_init_info;

mod thread;
mod tid_allocator;

mod idle;

pub use process::Process;
pub use process::State as ProcessState;
pub use process::Stats as ProcessStats;
pub use process_init_info::ProcessInitInfo;

pub use thread::Thread;

pub static PROCESS_TABLE: spin::RwLock<BTreeMap<usize, Arc<Process>>> 
    = spin::RwLock::new(BTreeMap::new());

pub static THREAD_TABLE: spin::RwLock<BTreeMap<usize, Arc<Thread>>> 
    = spin::RwLock::new(BTreeMap::new());

pub fn init() {
    pid_allocator::init();
    tid_allocator::init();
    idle::init();
}

pub fn get_proc(pid: usize) -> Option<Arc<Process>> 
  { PROCESS_TABLE.read().get(&pid).cloned() }

pub fn get_thread(tid: usize) -> Option<Arc<Thread>> 
  { THREAD_TABLE.read().get(&tid).cloned() }

pub fn add_proc(proc: Process) 
  { PROCESS_TABLE.write().insert(proc.get_pid(), Arc::new(proc) ); }

pub fn remove_proc(pid: usize) 
  { PROCESS_TABLE.write().remove(&pid); }

pub fn add_thread(thread: Thread) 
  { THREAD_TABLE.write().insert(thread.get_tid(), Arc::new(thread) ); }

pub fn remove_thread(tid: usize) 
  { THREAD_TABLE.write().remove(&tid); }