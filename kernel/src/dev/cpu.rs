pub trait CPU {
    fn shutdown() -> !;
    fn reboot() -> !;
    fn get_id() -> usize;
    fn send_ipi(cpu_id: usize);
    fn halt();
}

use crate::arch::dev::cpu::CPUImpl;

pub fn shutdown() -> ! { CPUImpl::shutdown() }

pub fn reboot() -> ! { CPUImpl::reboot() }

pub fn get_id() -> usize { CPUImpl::get_id() }

pub fn send_ipi(cpu_id: usize) { CPUImpl::send_ipi(cpu_id) }

pub fn halt() { CPUImpl::halt() }