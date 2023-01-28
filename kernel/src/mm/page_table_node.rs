use alloc::rc::Rc;

use super::page::Page;
use super::page_allocator;

type NodeRef = Option<Rc<PageTableNode>>;
pub struct PageTableNode {
    parent: NodeRef,
    page: Page,
}

impl PageTableNode {
    pub fn new(page: Page, parent: NodeRef) -> Self {
        Self {
            parent,
            page,
        }
    }

    pub fn alloc(parent: NodeRef) -> Self {
        Self {
            parent,
            page: page_allocator::alloc().unwrap()
        }
    }
} 
