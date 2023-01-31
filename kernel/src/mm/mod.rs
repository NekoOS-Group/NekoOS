pub mod page;
pub mod page_allocator;
pub mod page_table;
pub mod vm_segment;
pub mod vm_manager;

pub use page::Page;
pub use page_table::PageFlagImpl as PageFlag;
pub use page_table::PageTableImpl as PageTable;

pub use vm_segment::MapType;
pub use vm_segment::MapPermission;
pub use vm_segment::SegmentImpl as Segment;
pub use vm_manager::VmManagerImpl as VmManager;

//static KERNEL_SPACE: Mutex<vm_manager::VmManagerImpl> = None;

pub fn init(memory: &fdt::standard_nodes::Memory) {
    page_allocator::init(memory);
    page_allocator::test();
    crate::println!("[Neko] memory init to do!");
}