use super::super::sbi;

pub struct CPUImpl;

impl crate::dev::cpu::CPU for CPUImpl {
    fn shutdown() -> ! {
        sbi::shutdown();
    }
    
    fn reboot() -> ! {
        sbi::shutdown();
    }
    
    fn get_id() -> usize {
        super::super::register::get_tp()
    }
    
    fn send_ipi(cpu_id: usize) {
        sbi::send_ipi(1 << cpu_id);
    }
    
    fn halt() {
        unsafe { riscv::asm::wfi() }
    }
}