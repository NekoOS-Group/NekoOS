use riscv::register::sstatus::Sstatus;
pub struct Context {
    pub x : [usize; 32],
    pub sstatus : Sstatus,
    pub sepc: usize
}