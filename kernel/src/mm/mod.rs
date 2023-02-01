mod page;
mod vm_segment;
mod vm_manager;

pub mod page_table;
pub mod page_allocator;
pub mod kernel_space;

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

pub fn init(memory: &fdt::standard_nodes::Memory) {
    page_allocator::init(memory);
    page_allocator::test();
    kernel_space::init(memory);
    //kernel_space::on();
    //kernel_space::test();
}