use crate::mm;
use super::PageTableImpl;
use super::PageTableEntryImpl;
use mm::page_table::PageTableEntry;

impl mm::page_table::PageTable<PageTableEntryImpl> for PageTableImpl {
    fn activate(&self) {
        info!( "switch page table: root @ Page<{:#x}>", self.get_root().get_ppn() );
        unsafe {
            use riscv::register::satp::{self, Mode};
            satp::set(Mode::Sv39, 0, self.get_root().get_ppn());
            riscv::asm::sfence_vma_all();
        }
    }
}

fn dfs(ppn: usize, vpn: usize, deep: usize, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let node = super::PageTableNodeImpl::new(mm::Page{ppn});
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

impl core::fmt::Debug for PageTableImpl {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        dfs(self.get_root().get_ppn(), 0, 0, f)
    }
}
