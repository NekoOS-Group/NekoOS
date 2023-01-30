mod page;
mod page_allocator;
mod page_table;
mod page_table_node;
mod page_table_entry;
mod vm_segment;
mod vm_manager;

pub fn init(memory: &fdt::standard_nodes::Memory) {
    page_allocator::init(memory);
    page_allocator::test();
    crate::println!("[Neko] memory init to do!");
}