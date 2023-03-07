mod page;
mod vm_segment;
mod vm_space;
mod kernel_stack;

pub mod kernel_heap;
pub mod kernel_space;
pub mod page_table;
pub mod page_allocator;

pub use page::Page;
pub use vm_segment::MapType;
pub use vm_segment::MapPermission;
pub use kernel_stack::KernelStack;

pub type PageTable      = crate::arch::mm::PageTableImpl;
pub type PageTableEntry = crate::arch::mm::PageTableEntryImpl;
pub type PageAllocator  = page_allocator::PageAllocatorImpl;
pub type Segment        = vm_segment::SegmentImpl;
pub type VmSpace        = vm_space::VmSpaceImpl<PageTable>;

use crate::config::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeapWithRescue as Heap;

#[global_allocator]
static mut KERNEL_HEAP: Heap<32> = Heap::new( kernel_heap::enhence );
static mut KERNEL_HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

static mut GLOBAL_ALLOCATOR: Option<PageAllocator> = None;
static mut KERNEL_SPACE: Option<VmSpace> = None;

pub fn init(memory: &fdt::standard_nodes::Memory) {
    kernel_heap::init();
    kernel_heap::test();
    page_allocator::init(memory);
    page_allocator::test();
    kernel_space::init(memory);
    kernel_space::test();
}