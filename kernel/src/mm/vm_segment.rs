#![allow(unused)]
use alloc::collections::BTreeMap;

use crate::{mm, config};
use crate::mm::page_table::{ PageFlag, PageTable };

bitflags! {
    pub struct MapPermission : u8 {
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

#[allow(unused)]
impl MapPermission {
    pub fn is_readable(&self) -> bool {
        self.contains( MapPermission::R )
    }
    pub fn is_writable(&self) -> bool {
        self.contains( MapPermission::W )
    }
    pub fn is_executable(&self) -> bool {
        self.contains( MapPermission::X )
    }
    pub fn is_user_accessable(&self) -> bool {
        self.contains( MapPermission::U )
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
    permission: MapPermission,
    name: &'static str
}

impl SegmentImpl {
    pub fn new(name: &'static str, vpn_l: usize, vpn_r: usize, map_type: MapType, permission: MapPermission) -> Self {
        match map_type {
            MapType::Framed => {
                info!( 
                    "map {: <10} {:?} [{:#x}, {:#x}) -> physic pages {:?}",
                    name,
                    permission,
                    vpn_l * config::PAGE_SIZE,
                    vpn_r * config::PAGE_SIZE,
                    permission
                )
            }
            MapType::Linear { offset } => {
                info!( 
                    "map {: <10} [{:#x}, {:#x}) -> [{:#x}, {:#x}) ({} pages) {:?}", 
                    name,
                    vpn_l * config::PAGE_SIZE as usize,
                    vpn_r * config::PAGE_SIZE as usize,
                    vpn_l * config::PAGE_SIZE as usize - offset,
                    vpn_r * config::PAGE_SIZE as usize - offset,
                    vpn_r - vpn_l,
                    permission,
                );
            }
        }
        Self { name, vpn_l, vpn_r, pages: BTreeMap::new(), map_type, permission }
    }

    pub fn copy_from(another: &SegmentImpl) -> Self {
        Self { 
            name: another.name,
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
                ppn = vpn - offset / config::PAGE_SIZE;
            }
            MapType::Framed => {
                let page = mm::page_allocator::alloc().unwrap();
                ppn = page.ppn;
                self.pages.insert(vpn, page);
            }
        }

        page_table.map(vpn, ppn, 1, mm::PageFlag::from_permission(self.permission));
    }

    pub fn unmap_one(&mut self, page_table: &mut mm::PageTable, vpn: usize) {
        assert!( vpn >= self.vpn_l && vpn < self.vpn_r );

        match self.map_type {
            MapType::Framed => {
                self.pages.remove(&vpn);
            }
            _ => {}
        }
        
        page_table.unmap(vpn, 1);
    }

    pub fn map_all(&mut self, page_table: &mut mm::PageTable) {
        match self.map_type {
            MapType::Linear{ offset } => {
                page_table.map(
                    self.vpn_l, 
                    self.vpn_l - offset / config::PAGE_SIZE,
                    self.vpn_r - self.vpn_l, 
                    mm::PageFlag::from_permission(self.permission)
                );
            }
            MapType::Framed => {
                for vpn in self.vpn_l..self.vpn_r {
                    self.map_one(page_table, vpn);
                }
            }
        }
    }

    pub fn unmap_all(&mut self, page_table: &mut mm::PageTable) {
        page_table.unmap(self.vpn_l, self.vpn_r - self.vpn_r)
    }

    pub fn fetch_page(&mut self, page_table: &mut mm::PageTable,vpn: usize) -> &mm::Page {
        if let None = self.pages.get(&vpn) {
            self.map_one(page_table, vpn);
        }
        self.pages.get(&vpn).unwrap()
    }

    pub fn copy_data(&mut self, page_table: &mut mm::PageTable, data: &[u8]) {
        let mut start = 0usize;
        for vpn in self.vpn_l..self.vpn_r {
            let page = self.fetch_page(page_table, vpn);
            page.set_bytes(&data[start..data.len().min(start + config::PAGE_SIZE)]);
            start += config::PAGE_SIZE;
            if start >= data.len() { break; }
        }
    }
}