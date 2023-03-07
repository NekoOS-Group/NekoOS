use alloc::collections::BTreeMap;

use super::PageTableEntry;
use super::PageTableNode as Node;

use crate::mm;
use crate::println;

pub struct PageTableTemplate<const LEVEL: usize, const ORDER: usize, T> 
    where T: PageTableEntry + 'static
{
    root: Node<T>, 
    nodes: BTreeMap<usize, Node<T>>
}

impl<const LEVEL: usize, const ORDER: usize, T> PageTableTemplate<LEVEL, ORDER, T> 
    where T: PageTableEntry
{    
    fn insert_entry(&mut self, vpn: usize, entry: T, deep: usize) {
        let vpn = vpn >> ((LEVEL - deep) * ORDER);
        let indeces = 
            (1..deep).map(|x| (vpn >> (ORDER * x)) & ((1 << ORDER) - 1)).rev();

        let mut node = &mut self.root;
        node.size += 1;
        for index in indeces {
            let mut nxt_ppn = node.get_entry(index).get_ppn();
            if nxt_ppn == 0 {
                let new_node = Node::new_inner(&mut *node, index);
                nxt_ppn = new_node.get_ppn();
                self.nodes.insert(new_node.get_ppn(), new_node);
            }
            let nxt = self.nodes.get_mut(&nxt_ppn).unwrap();
            node = nxt; 
            node.size += 1;
        }
        node.set_entry(vpn & ((1 << ORDER) - 1), entry);
    }

    fn remove_entry(&mut self, vpn: usize, deep: usize) {
        let vpn = vpn >> ((LEVEL - deep) * ORDER);
        let indeces = 
            (1..deep).map(|x| (vpn >> (ORDER * (x))) & ((1 << ORDER) - 1));
            
        let mut node = &mut self.root;
        node.size -= 1;
        for index in indeces {
            let nxt_entry = node.get_entry(index);
            if node.size == 0 {
                let ppn = node.get_ppn();
                self.nodes.remove(&ppn);
            }
            let nxt = self.nodes.get_mut(&nxt_entry.get_ppn()).unwrap();
            node = nxt; node.size -= 1;
        }
        node.set_entry(vpn & ((1 << ORDER) - 1), PageTableEntry::new_empty());
    }

    fn query_entry(&self, vpn: usize, deep: usize) -> Option<T> {
        let vpn = vpn >> ((LEVEL - deep) * ORDER);
        let indeces = 
            (0..deep).map(|x| (vpn >> (ORDER * x)) & ((1 << ORDER) - 1));

        let mut node = &self.root;
        for index in indeces {
            if ! node.get_entry(index).is_valid() { return None; }
            let nxt = self.nodes.get(&node.get_entry(index).get_ppn());
            if let Some(nxt) = nxt {
                node = nxt;
            } else {
                return Some(node.get_entry(index));
            }
        }
        None
    }

    pub fn get_root<'a>(&'a self) -> &'a Node<T> {
        &self.root
    }
}

impl<const LEVEL: usize, const ORDER: usize, T> 
    super::PageTableInner<T> for PageTableTemplate<LEVEL, ORDER, T> 
        where T: PageTableEntry
{
    fn new() -> Self {
        Self {
            root: Node::new_alloc(),
            nodes: BTreeMap::new()
        }
    }

    fn map(&mut self, vpn: usize, ppn: usize, length: usize, permissions: mm::MapPermission) {
        let mut offset = 0;
        while offset < length {
            let vpn = vpn + offset;
            let ppn = ppn + offset;
            let deep = LEVEL - (vpn & (!vpn + 1)).ilog2().min( (length - offset).ilog2() ) as usize / ORDER;
            if let Some(_) = Self::query_entry(&self, vpn, deep) { panic!( "map vm twice"); }
            self.insert_entry(vpn, T::new(ppn, permissions), deep);
            offset += 1 << ((LEVEL - deep) * ORDER);
        }
    }

    fn unmap(&mut self, vpn: usize, length: usize) {
        let mut offset = 0;
        while offset < length {
            let vpn = vpn + offset;
            let deep = LEVEL - (vpn & (!vpn + 1)).ilog2().min( (length - offset).ilog2() ) as usize / ORDER;
            if let None = Self::query_entry(&self, vpn, deep) { panic!( "unmap free vm" ); }
            self.remove_entry(vpn, deep);
            offset += 1;
        }
    }

    fn query(&self, vpn: usize) -> Option<T> {
        let mut node = &self.root;
        let mut vpn = vpn;
        for deep in 1..=LEVEL {
            let index = ( vpn >> ((LEVEL - deep) * ORDER) ) & ((1 << ORDER) - 1);
            vpn ^= index << ((LEVEL - deep) * ORDER);
            if !node.get_entry(index).is_valid() { return None }
            if node.get_entry(index).is_leaf() {
                return Some(PageTableEntry::new(
                    node.get_entry(index).get_ppn() | vpn,
                    node.get_entry(index).get_permission()
                ))
            }
            node = self.nodes.get(&node.get_entry(index).get_ppn()).unwrap();
        }
        None
    }

}