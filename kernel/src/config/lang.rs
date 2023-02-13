#[cfg(debug_assertions)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = _info.location() {
        crate::println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            _info.message().unwrap()
        );
    } else {
        crate::println!("Panicked: {}", _info.message().unwrap());
    }
    crate::debug::backtrace();
    crate::sbi::shutdown();
}   


#[cfg(not(debug_assertions))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    crate::sbi::shutdown();
    crate::println!("Neko Panicked: {}", _info.message().unwrap());
}
