use crate::mm::page::Page;

mod stack_allocator;
mod buddy_allocator;

pub type PageAllocatorImpl = buddy_allocator::BuddyAllocator;

trait PageAllocator {
    fn new() -> Self;
    fn add(&mut self, l: usize, r:usize);
    fn alloc(&mut self) -> Option<Page>;
    fn dealloc(&mut self, ppn: usize);
}

use crate::mm::GLOBAL_ALLOCATOR;

pub fn alloc() -> Option<Page> {
    unsafe{ if let Some(ref mut inner) = GLOBAL_ALLOCATOR {
        inner.alloc()
    } else { None } }
}

pub fn dealloc(ppn: usize) {
    unsafe{ if let Some(ref mut inner) = GLOBAL_ALLOCATOR {
        inner.dealloc(ppn);
    } else {} }}

pub fn init(memory: &fdt::standard_nodes::Memory) {
    use crate::config::{skernel, ekernel, PAGE_SIZE, MEMORY_START_ADDRESS, PHYSICAL_MEMORY_OFFSET};
    info!( 
        "memory detect: region [{:#x}, {:#x}) ({} pages) reserved", 
        MEMORY_START_ADDRESS,
        ekernel as usize - PHYSICAL_MEMORY_OFFSET,
        (ekernel as usize - MEMORY_START_ADDRESS - PHYSICAL_MEMORY_OFFSET) / PAGE_SIZE
    );
    unsafe {
        GLOBAL_ALLOCATOR = Some(PageAllocatorImpl::new())
    }
    for region in memory.regions() {
        let mut l = region.starting_address as usize;
        let r = region.starting_address as usize + region.size.unwrap();
        if l <= skernel as usize - PHYSICAL_MEMORY_OFFSET && r >= ekernel as usize - PHYSICAL_MEMORY_OFFSET {
            l = ekernel as usize - PHYSICAL_MEMORY_OFFSET;
        }
        add(l / PAGE_SIZE, r / PAGE_SIZE);
    }
    crate::println!("[Neko] page allocator inited.");
}

pub fn add(l: usize, r:usize) {
    use crate::config::PAGE_SIZE;
    info!( "memory detect: region [{:#x}, {:#x}) ({} pages) avaliable", 
        l * PAGE_SIZE, 
        r * PAGE_SIZE,
        r - l
    );
    unsafe{ if let Some(ref mut inner) = GLOBAL_ALLOCATOR {
        inner.add(l, r);
    } else {} };
}

#[cfg(debug_assertions)]
pub fn test() {
    use alloc::vec::Vec;
    let mut v: Vec<Page> = Vec::new();
    for _ in 0..5 {
        let frame = alloc().unwrap();
        v.push(frame);
    }
    v.clear();
    for _ in 0..5 {
        let frame = alloc().unwrap();
        v.push(frame);
    }
    drop(v);
    info!("frame allocator test passed!");
}

#[cfg(not(debug_assertions))]
pub fn test() {}
