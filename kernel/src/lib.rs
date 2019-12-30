#![no_std]
#![feature(core_intrinsics, lang_items, asm)]

use core::panic::PanicInfo;
use raspi4::uart;

#[no_mangle]
pub extern "C" fn kernel_main() {
    uart::init();
    uart::puts("Hello, world");
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
