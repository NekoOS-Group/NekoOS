use crate::mm;
use crate::mm::KERNEL_SPACE;
use super::page_table::PageTable;

pub fn init(memory: &fdt::standard_nodes::Memory) {
    use crate::config::{ skernel, ekernel, stext, etext, srodata, erodata, sdata, edata, sbss, ebss, PHYSICAL_MEMORY_OFFSET, PAGE_SIZE };
    unsafe {
        KERNEL_SPACE = Some(mm::VmManager::new());
        if let Some(inner) = &mut KERNEL_SPACE {
            inner.push(
                mm::Segment::new(
                    ".text",
                    stext as usize / PAGE_SIZE,
                    etext as usize / PAGE_SIZE,
                    mm::MapType::Linear { offset: PHYSICAL_MEMORY_OFFSET },
                    mm::MapPermission::X
                ), 
                None
            );
            inner.push(
                mm::Segment::new(
                    ".rodata",
                    srodata as usize / PAGE_SIZE,
                    erodata as usize / PAGE_SIZE,
                    mm::MapType::Linear { offset: PHYSICAL_MEMORY_OFFSET },
                    mm::MapPermission::R
                ), 
                None
            );
            inner.push(
                mm::Segment::new(
                    ".data",
                    sdata as usize / PAGE_SIZE,
                    edata as usize / PAGE_SIZE,
                    mm::MapType::Linear { offset: PHYSICAL_MEMORY_OFFSET },
                    mm::MapPermission::R | mm::MapPermission::W
                ), 
                None
            );
            inner.push(
                mm::Segment::new(
                    ".bss",
                    sbss as usize / PAGE_SIZE,
                    ebss as usize / PAGE_SIZE,
                    mm::MapType::Linear { offset: PHYSICAL_MEMORY_OFFSET },
                    mm::MapPermission::R | mm::MapPermission::W
                ), 
                None
            );
            for region in memory.regions() {
                let mut l = region.starting_address as usize + PHYSICAL_MEMORY_OFFSET;
                let r = region.starting_address as usize + region.size.unwrap() + PHYSICAL_MEMORY_OFFSET;
                if l <= skernel as usize && r >= ekernel as usize {
                    l = ekernel as usize;
                }
                inner.push(
                    mm::Segment::new(
                        "region",
                        l / PAGE_SIZE,
                        r / PAGE_SIZE,
                        mm::MapType::Linear { offset: PHYSICAL_MEMORY_OFFSET },
                        mm::MapPermission::R | mm::MapPermission::W
                    ), 
                    None
                );
            }
        }
    }
    on();
    crate::println!("[Neko] kernel space inited.");
}

pub fn on() {
    unsafe {
        if let Some(inner) = &mut KERNEL_SPACE {
            inner.get_page_table().activate();
        }
    }
}

#[cfg(debug_assertions)]
pub fn test() {
    use crate::config::{ stext, etext, srodata, erodata, sdata, edata, PAGE_SIZE };
    unsafe {
        if let Some(kernel_space) = &mut KERNEL_SPACE {
            let mid_text = stext as usize / 2 + etext as usize / 2;
            let mid_rodata = srodata as usize / 2+ erodata as usize/ 2;
            let mid_data = sdata as usize / 2 + edata as usize / 2;
            assert!(!kernel_space
                .get_page_table()
                .query_permission(mid_text / PAGE_SIZE)
                .is_readable());
            assert!(!kernel_space
                .get_page_table()
                .query_permission(mid_rodata / PAGE_SIZE)
                .is_writable());
            assert!(!kernel_space
                .get_page_table()
                .query_permission(mid_data / PAGE_SIZE)
                .is_executable());
            info!("kernel space remap test passed!");
        }
    }
}

#[cfg(not(debug_assertions))]
pub fn test() {}