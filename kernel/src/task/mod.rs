use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use riscv::interrupt::Mutex;

use crate::algorithm::allocator;
use crate::config;
use crate::dev;
use crate::println;

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

pub type ProcessRef = Arc<spin::Mutex<Process>>;
pub type ThreadRef  = Arc<spin::Mutex<Thread>>;

pub static PID_ALLOCATOR: spin::Mutex<Option<allocator::BuddyAllocator>> = spin::Mutex::new(None);
pub static TID_ALLOCATOR: spin::Mutex<Option<allocator::BuddyAllocator>> = spin::Mutex::new(None);

pub static PROCESS_POOL: spin::RwLock<BTreeMap<usize, ProcessRef>> 
    = spin::RwLock::new(BTreeMap::new());

pub static THREAD_POOL: spin::RwLock<BTreeMap<usize, ThreadRef>> 
    = spin::RwLock::new(BTreeMap::new());

const INIT: Option<Processor> = None;
pub static mut PROCESSOR_POOL: [Option<Processor>; config::MAX_CPU_CORE]
    = [INIT; config::MAX_CPU_CORE];

pub fn init() {
    pid_allocator::init();
    tid_allocator::init();
    idle::init();
    println!( "[Neko] task inited(todo)" );
}

pub fn get_proc(pid: usize) -> Option<ProcessRef> 
    { PROCESS_POOL.read().get(&pid).cloned() }

pub fn get_thread(tid: usize) -> Option<ThreadRef> 
    { THREAD_POOL.read().get(&tid).cloned() }

pub fn remove_proc(pid: usize) 
    { PROCESS_POOL.write().remove(&pid); }

pub fn remove_thread(tid: usize) 
    { THREAD_POOL.write().remove(&tid); }

pub fn current_thread() -> Option<ThreadRef> {  
    let id = dev::cpu::get_id();
    unsafe{ PROCESSOR_POOL[id].as_ref().map(|v| v.current_thread.clone() ) }
}

pub fn current_processor() -> &'static mut Processor {
    let id = dev::cpu::get_id();
    unsafe{ PROCESSOR_POOL[id].as_mut().unwrap() }
}