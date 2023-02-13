use alloc::collections::BTreeMap;

use super::page_table_entry::PageTableEntry;
use super::page_table_node::Node;

use crate::mm;

pub struct PageTable<const LEVEL: usize, const ORDER: usize> {
    root: Node, 
    nodes: BTreeMap<usize, Node>
}

impl<const LEVEL: usize, const ORDER: usize> PageTable<LEVEL, ORDER> {    
    fn insert_entry(&mut self, vpn: usize, entry: PageTableEntry, deep: usize) {
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

    fn query_entry(&self, vpn: usize, deep: usize) -> Option<PageTableEntry>{
        let vpn = vpn >> ((LEVEL - deep) * ORDER);
        let indeces = 
            (0..deep).map(|x| (vpn >> (ORDER * x)) & ((1 << ORDER) - 1));

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

impl<const LEVEL: usize, const ORDER: usize> 
    mm::page_table::PageTable for PageTable<LEVEL, ORDER> 
{
    fn new() -> Self {
        PageTable {
            root: Node::new_alloc(),
            nodes: BTreeMap::new()
        }
    }

    fn map(&mut self, vpn: usize, ppn: usize, length: usize, flags: mm::PageFlag) {
        let mut offset = 0;
        while offset < length {
            let vpn = vpn + offset;
            let ppn = ppn + offset;
            let deep = LEVEL - (vpn & (!vpn + 1)).ilog2().min( (length - offset).ilog2() ) as usize / ORDER;
            if let Some(_) = Self::query_entry(&self, vpn, deep) { panic!( "map vm twice"); }
            Self::insert_entry(self, vpn, PageTableEntry::new(ppn, flags), deep);
            offset += 1 << ((LEVEL - deep) * ORDER);
        }
    }

    fn unmap(&mut self, vpn: usize, length: usize) {
        let mut offset = 0;
        while offset < length {
            let vpn = vpn + offset;
            let deep = LEVEL - (vpn & (!vpn + 1)).ilog2().min( (length - offset).ilog2() ) as usize / ORDER;
            if let None = Self::query_entry(&self, vpn, deep) { panic!( "unmap free vm" ); }
            Self::remove_entry(self, vpn, deep);
            offset += 1;
        }
    }

    fn activate(&self) {
        info!( "switch page table: root @ Page<{:#x}>", self.root.get_ppn() );
        let satp = self.root.get_ppn() | 8usize << 60;
        unsafe {
            riscv::register::satp::write(satp);
            riscv::asm::sfence_vma_all();
        }
    }

    fn query_ppn(&self, vpn: usize) -> Option<usize>{
        if let Some(inner) = self.query_entry(vpn, 3) {
            Some(inner.get_ppn())
        } else {
            None
        }
    }

    fn query_permission(&self, vpn: usize) -> mm::MapPermission {
        if let Some(inner) = self.query_entry(vpn, 3) {
            mm::MapPermission::from_bits(inner.get_flags().bits() & 0b11110).unwrap()
        } else {
            mm::MapPermission::from_bits(0).unwrap()
        }
    }
}

fn dfs(ppn: usize, vpn: usize, deep: usize, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let node = Node::new(mm::Page{ppn});
    for _ in 0..deep { f.write_fmt(format_args!("   |"))?; }
    let mut vp_l = vpn * 4096;
    let mut vp_r = vp_l + (1 << (9 * (3 - deep))) * 4096;
    if vp_l & (3usize << 38) != 0 { vp_l -= 1usize << 39; }
    if vp_r & (3usize << 38) != 0 { vp_r -= 1usize << 39; }
    if vp_r == 0 { vp_r = usize::MAX; }
    f.write_fmt(format_args!("{:03} [{:#x}, {:#x}) @ Page<{:#x}>\n", vpn >> (27 - deep * 9) & 511, vp_l, vp_r, ppn) )?;
    for i in 0..512 {
        let nxt = node.get_entry(i);
        if nxt.is_valid() {
            if nxt.is_writable() || nxt.is_readable() || nxt.is_executable() {
                let vpn = vpn | (i << (18 - deep * 9));
                for _ in 0..deep+1 { f.write_fmt(format_args!("   |"))?; }
                let mut vp_l = vpn * 4096;
                let mut vp_r = vp_l + (1 << (9 * (2 - deep))) * 4096;
                if vp_l & (1usize << 38) != 0 { vp_l -= 3usize << 39; }
                if vp_r & (1usize << 38) != 0 { vp_r -= 3usize << 39; }
                if vp_r == 0 { vp_r = usize::MAX; }
                f.write_fmt(format_args!( "{:03} [{:#x}, {:#x}) -> {:?}\n", i, vp_l, vp_r, nxt))?;
            } else {
                dfs( nxt.get_ppn(), vpn | (i << (18 - deep * 9)) , deep + 1, f)?;
            }
        }
    }
    core::mem::forget(node);
    Ok(())
}

impl<const LEVEL: usize, const ORDER: usize> 
    core::fmt::Debug for PageTable<LEVEL, ORDER> 
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        dfs(self.root.get_ppn(), 0, 0, f)
    }
}