mod page;
mod page_allocator;
mod page_table;
mod page_table_node;
mod page_table_entry;

pub fn init() {
    page_allocator::init();
    page_allocator::test();
    crate::println!("[Neko] memory init to do!");
}