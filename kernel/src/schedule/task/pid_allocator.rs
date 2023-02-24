use crate::config;

type PidAllocatorImpl = buddy_system_allocator::LockedFrameAllocator;

pub static mut PID_ALLOCATOR: Option<PidAllocatorImpl> = None;

pub fn init() {
    unsafe {
        if let Some(inner) = &PID_ALLOCATOR {
            inner.lock().insert(0..config::MAX_PROCESS);
        }
    }
}

pub fn alloc() -> Option<usize> {
    unsafe {
        if let Some(inner) = &PID_ALLOCATOR {
            inner.lock().alloc(1)
        } else { None }
    }
}

pub fn dealloc(pid: usize) {
    unsafe {
        if let Some(inner) = &PID_ALLOCATOR {
            inner.lock().dealloc(pid, 0)
        }
    }
}