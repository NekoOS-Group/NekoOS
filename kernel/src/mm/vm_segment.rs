use alloc::collections::BTreeMap;

use crate::mm;
use crate::mm::page_table::{ PageFlag, PageTable };

bitflags! {
    pub struct MapPermission : u8 {
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

#[derive(Copy, Clone)]
pub enum MapType {
    Linear { offset: usize },
    Framed,
}

pub struct SegmentImpl {
    vpn_l: usize,
    vpn_r: usize,
    pages: BTreeMap<usize, mm::Page>,
    map_type: MapType,
    permission: MapPermission
}

impl SegmentImpl {
    pub fn new(vpn_l: usize, vpn_r: usize, map_type: MapType, permission: MapPermission) -> Self {
        Self { vpn_l, vpn_r, pages: BTreeMap::new(), map_type, permission }
    }

    pub fn copy_from(another: &SegmentImpl) -> Self {
        Self { 
            vpn_l: another.vpn_l, 
            vpn_r: another.vpn_r, 
            pages: BTreeMap::new(), 
            map_type: another.map_type, 
            permission: another.permission 
        }
    }
}

impl SegmentImpl {
    pub fn map_one(&mut self, page_table: &mut mm::PageTable, vpn: usize) {
        assert!( vpn >= self.vpn_l && vpn < self.vpn_r );

        let ppn;
        match self.map_type {
            MapType::Linear { offset } => {
                ppn = vpn - offset;
            }
            MapType::Framed => {
                let page = mm::page_allocator::alloc().unwrap();
                ppn = page.ppn;
                self.pages.insert(vpn, page);
            }
        }

        page_table.map(vpn, ppn, mm::PageFlag::from_permission(self.permission) );
    }

    pub fn unmap_one(&mut self, page_table: &mut mm::PageTable, vpn: usize) {
        assert!( vpn >= self.vpn_l && vpn < self.vpn_r );

        match self.map_type {
            MapType::Framed => {
                self.pages.remove(&vpn);
            }
            _ => {}
        }
        
        page_table.unmap(vpn);
    }

    pub fn map_all(&mut self, page_table: &mut mm::PageTable) {
        for vpn in self.vpn_l..self.vpn_r {
            self.map_one(page_table, vpn);
        }
    }

    pub fn unmap_all(&mut self, page_table: &mut mm::PageTable) {
        for vpn in self.vpn_l..self.vpn_r {
            self.unmap_one(page_table, vpn);
        }
    }
}