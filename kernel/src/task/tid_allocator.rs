use crate::algorithm::allocator::BuddyAllocator;
use crate::algorithm::allocator::Allocator;
use crate::config;

use super::TID_ALLOCATOR;

pub fn init() {
    unsafe {
        TID_ALLOCATOR = Some( BuddyAllocator::new() );
        TID_ALLOCATOR.as_mut().map( |inner| 
            { inner.add(0, config::MAX_THREAD); }
        );
    }
}

pub fn alloc() -> Option<usize> {
    unsafe {
        TID_ALLOCATOR.as_mut().map( |inner| 
            { inner.alloc() }
        ).unwrap()
    }
}

pub fn dealloc(pid: usize) {
    unsafe {
        TID_ALLOCATOR.as_mut().map( |inner| 
            { inner.dealloc(pid) }
        );
    }
}