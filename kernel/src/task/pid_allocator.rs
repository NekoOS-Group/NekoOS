use crate::algorithm::allocator::BuddyAllocator;
use crate::algorithm::allocator::Allocator;
use crate::config;
use crate::println;

use super::PID_ALLOCATOR;

pub fn init() {
    unsafe {
        PID_ALLOCATOR = Some( BuddyAllocator::new() );
        PID_ALLOCATOR.as_mut().map( |inner| 
            { inner.add(0, config::MAX_THREAD); }
        );
    }
}

pub fn alloc() -> Option<usize> {
    unsafe {
        PID_ALLOCATOR.as_mut().map( |inner| 
            { inner.alloc() }
        ).unwrap()
    }
}

pub fn dealloc(pid: usize) {
    unsafe {
        PID_ALLOCATOR.as_mut().map( |inner| 
            { inner.dealloc(pid) }
        );
    }
}