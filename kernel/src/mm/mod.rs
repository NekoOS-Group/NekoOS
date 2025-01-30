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
pub type KernelHeap     = buddy_system_allocator::LockedHeap<32>;

#[global_allocator]
static KERNEL_HEAP: KernelHeap = KernelHeap::new();

static GLOBAL_ALLOCATOR: spin::Mutex<Option<PageAllocator>> = spin::Mutex::new(None);
static KERNEL_SPACE: spin::Mutex<Option<VmSpace>> = spin::Mutex::new(None);

pub fn init(memory: &fdt::standard_nodes::Memory) {
    kernel_heap::init();
    kernel_heap::test();
    page_allocator::init(memory);
    page_allocator::test();
    kernel_space::init(memory);
    kernel_space::test();
}