#![no_std]
#![no_main]
#![feature(core_intrinsics, lang_items, asm, global_asm)]

use core::panic::PanicInfo;
use raspi4::uart;

global_asm!(include_str!("start.S"));

#[no_mangle]
pub extern "C" fn kernel_main() {
    uart::init();
    uart::puts("Hello, kernel");
    loop {
        let ch = uart::readc();
        uart::putc(ch as u32);
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
