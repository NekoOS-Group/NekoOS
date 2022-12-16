use riscv::register::sstatus::{self, Sstatus, SPP};
pub struct TrapContext {
    pub x : [usize; 32],
    pub sstatus : Sstatus,
    pub sepc: usize
}