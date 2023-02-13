mod page;
mod vm_segment;
mod vm_manager;

pub mod kernel_heap;
pub mod kernel_space;
pub mod page_table;
pub mod page_allocator;

pub use page::Page;
pub use page_allocator::PageAllocatorImpl as PageAllocator;
pub use page_table::PageFlagImpl as PageFlag;
pub use page_table::PageTableImpl as PageTable;

pub use vm_segment::MapType;
pub use vm_segment::MapPermission;
pub use vm_segment::SegmentImpl as Segment;
pub use vm_manager::VmManagerImpl as VmManager;

static mut GLOBAL_ALLOCATOR: Option<PageAllocator> = None;
static mut KERNEL_SPACE: Option<VmManager> = None;

use crate::config::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeapWithRescue as Heap;

#[global_allocator]
static mut KERNEL_HEAP: Heap<32> = Heap::new( kernel_heap::enhence );
static mut KERNEL_HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

pub fn init(memory: &fdt::standard_nodes::Memory) {
    kernel_heap::init();
    kernel_heap::test();
    page_allocator::init(memory);
    page_allocator::test();
    kernel_space::init(memory);
    kernel_space::test();
}