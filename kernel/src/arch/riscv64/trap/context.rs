use riscv::register::sstatus::Sstatus;
pub struct TrapContext {
    pub x : [usize; 32],
    pub sstatus : Sstatus,
    pub sepc: usize
}