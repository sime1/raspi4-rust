#![no_std]
#![feature(core_intrinsics, lang_items, asm)]

use core::panic::PanicInfo;
use raspi4::uart;

#[no_mangle]
pub extern "C" fn kernel_main() {
    uart::init();
    let sp: u64;
    unsafe {
        asm!("mov $0, sp": "=r"(sp))
    }
    uart::puts("Hello, world. sp: 0x");
    print_number(sp);
    loop {
        let ch = uart::readc();
        uart::putc(ch as u32);
    }
}

fn print_number(n: u64) {
    for i in 0..16 {
        let digit = ((n << i * 4) >> 60) & 0xF;
        let ch = match digit {
            0x0 => '0' as u8,
            0x1 => '1' as u8,
            0x2 => '2' as u8,
            0x3 => '3' as u8,
            0x4 => '4' as u8,
            0x5 => '5' as u8,
            0x6 => '6' as u8,
            0x7 => '7' as u8,
            0x8 => '8' as u8,
            0x9 => '9' as u8,
            0xA => 'A' as u8,
            0xB => 'B' as u8,
            0xC => 'C' as u8,
            0xD => 'D' as u8,
            0xE => 'E' as u8,
            0xF => 'F' as u8,
            _ => '?' as u8
        };
        uart::putc(ch as u32)
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
