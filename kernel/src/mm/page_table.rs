use core::borrow::BorrowMut;
use core::cell::RefCell;
use core::ops::Deref;
use alloc::collections::BTreeMap;
use alloc::rc::Rc;

use super::page_table_entry::{PageTableEntry, self};
use super::page_table_node::Node;

pub struct PageTable {
    root: Rc<RefCell<Node>>, 
    nodes: Rc<RefCell<BTreeMap<usize, Node>>>
}

impl PageTable {
    pub fn new() -> Self {
        PageTable {
            root: Rc::new(RefCell::new(Node::new_alloc())),
            nodes: Rc::new(RefCell::new(BTreeMap::new()))
        }
    }
}

impl PageTable {
    pub fn map(&mut self, vpn: usize, ppn: usize, flags: super::page_table_entry::Flags) {
        let l1_index = (vpn >> 18) & 511;
        let l2_index = (vpn >> 9) & 511;
        let l3_index = vpn & 511;

        let node1 = self.nodes.clone();
        let mut node2 = node1.deref().borrow_mut();
        let node3 = self.nodes.clone();
        let mut node4 = node3.deref().borrow_mut();

        let root1 = self.root.clone();

        let mut l1 = root1.deref().borrow_mut();
        let l1_entry = l1.borrow_mut().get_entry(l1_index);
        
        if l1_entry.is_empty() {
            self.nodes.deref().borrow_mut().insert(l1_index, Node::new_inner(l1.borrow_mut(), l1_index));
        }

        let l2 = node2.get_mut(&l1_entry.get_ppn()).unwrap();
        let l2_entry = l2.get_entry(l2_index);
        
        if l2_entry.is_empty() {
            self.nodes.deref().borrow_mut().insert(l1_index,Node::new_inner(l2.borrow_mut(), l2_index ));
        }
        let l3 = node4.get_mut( &l2_entry.get_ppn() ).unwrap();
        let l3_entry = l3.get_entry(l3_index);
        
        if l3_entry.is_empty() {
            l1.size += 1;
            l2.size += 1;
            l3.size += 1;
        }

        l3.set_entry( l3_index, PageTableEntry::new(ppn, flags | page_table_entry::Flags::V ) );
    }

    pub fn unmap(&mut self, vpn: usize) {
        let l1_index = (vpn >> 18) & 511;
        let l2_index = (vpn >> 9) & 511;
        let l3_index = vpn & 511;

        let node1 = self.nodes.clone();
        let mut node2 = node1.deref().borrow_mut();
        let node3 = self.nodes.clone();
        let mut node4 = node3.deref().borrow_mut();

        let root1 = self.root.clone();

        let mut l1 = root1.deref().borrow_mut();
        let l1_entry = l1.borrow_mut().get_entry(l1_index);
        
        let l2 = if l1_entry.is_empty() {
            return;
        } else {
            node2.get_mut( &l1_entry.get_ppn() ).unwrap()
        };
        let l2_entry = l2.get_entry(l2_index);
        
        let l3 = if l2_entry.is_empty() {
            return;
        } else {
            node4.get_mut( &l2_entry.get_ppn() ).unwrap()
        };
        let l3_entry = l3.get_entry(l3_index);
        
        if l3_entry.is_empty() {
            l1.size -= 1;
            l2.size -= 1;
            l3.size -= 1;
            if l3.size == 0 { self.nodes.deref().borrow_mut().remove( &l2_entry.get_ppn() ); }
            if l2.size == 0 { self.nodes.deref().borrow_mut().remove( &l1_entry.get_ppn() ); }
        }
        l3.set_entry(
            l3_index, PageTableEntry::new_empty()
        );
    }

    pub fn active() {
        
    }
}