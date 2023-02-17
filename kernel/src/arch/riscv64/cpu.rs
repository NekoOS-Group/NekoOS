pub fn shutdown() -> ! {
    super::sbi::shutdown();
}

pub fn reboot() -> ! {
    super::sbi::shutdown();
}

pub fn get_id() -> usize {
    super::register::get_tp()
}