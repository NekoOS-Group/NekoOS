use core::alloc::Layout;

use buddy_system_allocator::Heap;

use crate::mm::KERNEL_HEAP;
use crate::config;

static mut HEAP_SPACE: [u8; config::KERNEL_HEAP_SIZE] = [0; config::KERNEL_HEAP_SIZE];

pub fn init() {
    unsafe {
        KERNEL_HEAP.lock().init(HEAP_SPACE.as_ptr() as usize, config::KERNEL_HEAP_SIZE);
    }
    crate::println!("[Neko] heap inited.");
}

pub fn enhence(_heap: &mut Heap<32>, _layout: &Layout) {
    panic!( "heap out of memory" );
}

#[cfg(debug_assertions)]
pub fn test() {
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    extern "C" {
        fn sbss();
        fn ebss();
    }
    let bss_range = sbss as usize..ebss as usize;
    let a = Box::new(5);
    assert_eq!(*a, 5);
    assert!(bss_range.contains(&(a.as_ref() as *const _ as usize)));
    drop(a);
    let mut v: Vec<usize> = Vec::new();
    for i in 0..500 {
        v.push(i);
    }
    for i in 0..500 {
        assert_eq!(v[i], i);
    }
    assert!(bss_range.contains(&(v.as_ptr() as usize)));
    drop(v);
    info!("kernel heap test passed!");
}

#[cfg(not(debug_assertions))]
pub fn test() {}
