use alloc::boxed::Box;
use super::page::Page;

mod stack_allocator;
mod buddy_allocator;

trait PageAllocator {
    fn new() -> Self;
    fn add(&mut self, l: usize, r:usize);
    fn alloc(&mut self) -> Option<Page>;
    fn dealloc(&mut self, ppn: usize);
}

type PageAllocatorImpl = buddy_allocator::BuddyAllocator;

pub static mut GLOABAL_ALLOCATOR: Option<Box<PageAllocatorImpl>> = None;

pub fn alloc() -> Option<Page> {
    unsafe{ if let Some(ref mut inner) = GLOABAL_ALLOCATOR {
        inner.alloc()
    } else { None } }
}

pub fn dealloc(ppn: usize) {
    unsafe{ if let Some(ref mut inner) = GLOABAL_ALLOCATOR {
        inner.dealloc(ppn);
    } else {} }}

pub fn init(memory: &fdt::standard_nodes::Memory) {
    use crate::config::{skernel, ekernel, PAGE_SIZE, MEMORY_START_ADDRESS};
    info!( 
        "memory detect: region [{:#x}, {:#x}) ({} pages) reserved", 
        MEMORY_START_ADDRESS,
        ekernel as usize,
        (ekernel as usize - MEMORY_START_ADDRESS) / PAGE_SIZE
    );
    unsafe {
        GLOABAL_ALLOCATOR = Some(Box::new(PageAllocatorImpl::new()))
    }
    for region in memory.regions() {
        let mut l = region.starting_address as usize;
        let r = region.starting_address as usize + region.size.unwrap();
        if l <= skernel as usize && r >= ekernel as usize {
            l = ekernel as usize;
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
    unsafe{ if let Some(ref mut inner) = GLOABAL_ALLOCATOR {
        inner.add(l, r);
    } else {} };
}

#[allow(unused)]
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
    debug!("frame_allocator_test passed!");
}