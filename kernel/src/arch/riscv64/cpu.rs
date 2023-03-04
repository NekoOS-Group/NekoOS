pub struct CPUImpl;

impl crate::dev::cpu::CPU for CPUImpl {
    fn shutdown() -> ! {
        super::sbi::shutdown();
    }
    
    fn reboot() -> ! {
        super::sbi::shutdown();
    }
    
    fn get_id() -> usize {
        super::register::get_tp()
    }
    
    fn send_ipi(cpu_id: usize) {
        super::sbi::send_ipi(1 << cpu_id);
    }
    
    fn halt() {
        unsafe { riscv::asm::wfi() }
    }
}