use alloc::collections::BTreeMap;
use alloc::sync::Arc;

mod process;
mod pid_allocator;
mod process_init_info;

mod thread;
mod tid_allocator;

mod processor;

mod task_scheduler;
mod idle;

pub use process::Process;
pub use process::State as ProcessState;
pub use process::Stats as ProcessStats;
pub use process_init_info::ProcessInitInfo;

pub use thread::Thread;

pub use processor::Processor;

pub static PROCESS_POOL: spin::RwLock<BTreeMap<usize, Arc<Process>>> 
    = spin::RwLock::new(BTreeMap::new());

pub static THREAD_POOL: spin::RwLock<BTreeMap<usize, Arc<Thread>>> 
    = spin::RwLock::new(BTreeMap::new());

pub fn init() {
    pid_allocator::init();
    tid_allocator::init();
    idle::init();
}

pub fn get_proc(pid: usize) -> Option<Arc<Process>> 
  { PROCESS_POOL.read().get(&pid).cloned() }

pub fn get_thread(tid: usize) -> Option<Arc<Thread>> 
  { THREAD_POOL.read().get(&tid).cloned() }

pub fn add_proc(proc: Process) 
  { PROCESS_POOL.write().insert(proc.get_pid(), Arc::new(proc) ); }

pub fn remove_proc(pid: usize) 
  { PROCESS_POOL.write().remove(&pid); }

pub fn add_thread(thread: Thread) 
  { THREAD_POOL.write().insert(thread.get_tid(), Arc::new(thread) ); }

pub fn remove_thread(tid: usize) 
  { THREAD_POOL.write().remove(&tid); }