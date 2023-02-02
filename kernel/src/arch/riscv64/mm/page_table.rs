use alloc::collections::BTreeMap;

use super::page_table_entry::PageTableEntry;
use super::page_table_node::Node;

use crate::mm;

pub struct PageTable {
    root: Node, 
    nodes: BTreeMap<usize, Node>
}

impl PageTable {
    fn insert_entry(&mut self, vpn: usize, entry: PageTableEntry) {
        let indeces = [(vpn >> 18) & 511, (vpn >> 9) & 511];
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
        node.set_entry(vpn & 511, entry);
    }

    fn remove_entry(&mut self, vpn: usize) {
        let indeces = [(vpn >> 18) & 511, (vpn >> 9) & 511];
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
        node.set_entry(vpn & 511, PageTableEntry::new_empty());
    }

    fn query_entry(&self, vpn: usize) -> Option<PageTableEntry>{
        let indeces = [(vpn >> 18) & 511, (vpn >> 9) & 511, vpn & 511];
        let mut node = &self.root;
        for index in indeces {
            if node.get_entry(index).is_empty() { return None; }
            let nxt = self.nodes.get(&node.get_entry(index).get_ppn());
            if let Some(nxt) = nxt {
                node = nxt;
            } else {
                return Some(node.get_entry(index));
            }
        }
        None
    }
}

impl mm::page_table::PageTable for PageTable {
    fn new() -> Self {
        PageTable {
            root: Node::new_alloc(),
            nodes: BTreeMap::new()
        }
    }

    fn map(&mut self, vpn: usize, ppn: usize, flags: mm::PageFlag) {
        if let Some(_) = Self::query_ppn(&self, vpn) { panic!( "map a vpn twice"); }
        Self::insert_entry(self, vpn, PageTableEntry::new(ppn, flags));
    }

    fn unmap(&mut self, vpn: usize) {
        if let None = Self::query_ppn(&self, vpn) { panic!( "unmap a not mapped vpn" ); }
        Self::remove_entry(self, vpn);
    }

    fn activate(&self) {
        info!( "switch page table: root <{:#x}>", self.root.get_ppn() );
        let satp = self.root.get_ppn() | 8usize << 60;
        unsafe {
            riscv::register::satp::write(satp);
            riscv::asm::sfence_vma_all();
        }
    }

    fn query_ppn(&self, vpn: usize) -> Option<usize>{
        if let Some(inner) = self.query_entry(vpn) {
            Some(inner.get_ppn())
        } else {
            None
        }
    }

    fn query_permission(&self, vpn: usize) -> mm::MapPermission {
        if let Some(inner) = self.query_entry(vpn) {
            mm::MapPermission::from_bits(inner.get_flags().bits() & 0b11110).unwrap()
        } else {
            mm::MapPermission::from_bits(0).unwrap()
        }
    }
}

fn dfs(ppn: usize, vpn: usize, deep: usize, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let node = Node::new(mm::Page{ppn});
    for _ in 0..deep { f.write_fmt(format_args!("   |"))?; }
    f.write_fmt(format_args!("<{:#x}>\n", ppn) )?;
    for i in 0..512 {
        let nxt = node.get_entry(i);
        if nxt.is_valid()  {
            if nxt.is_writable() || nxt.is_readable() || nxt.is_executable() {
                let mut vpn = vpn << 9 | i;
                if (vpn >> 26) & 1 != 0 { vpn -= 1usize << 27; }
                for _ in 0..deep+1 { f.write_fmt(format_args!("   |"))?; }
                f.write_fmt(format_args!( "{:#x} -> {:?}\n", vpn, nxt))?;
            } else {
                dfs( nxt.get_ppn(), vpn << 9 | i, deep + 1, f)?;
            }
        }
    }
    core::mem::forget(node);
    Ok(())
}

impl core::fmt::Debug for PageTable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        dfs(self.root.get_ppn(), 0, 0, f)
    }
}