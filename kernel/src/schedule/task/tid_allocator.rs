use crate::config;

type TidAllocatorImpl = buddy_system_allocator::LockedFrameAllocator;

pub static mut TID_ALLOCATOR: Option<TidAllocatorImpl> = None;

pub fn init() {
    unsafe {
        if let Some(inner) = &TID_ALLOCATOR {
            inner.lock().insert(0..config::MAX_THREAD);
        }
    }
}

pub fn alloc() -> Option<usize> {
    unsafe {
        if let Some(inner) = &TID_ALLOCATOR {
            inner.lock().alloc(1)
        } else { None }
    }
}

pub fn dealloc(pid: usize) {
    unsafe {
        if let Some(inner) = &TID_ALLOCATOR {
            inner.lock().dealloc(pid, 0)
        }
    }
}