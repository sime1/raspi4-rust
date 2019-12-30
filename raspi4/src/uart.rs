use super::mbox;
use super::mmio;
use super::mmio::enable_gpio_pin;
use super::mmio::GPIOFunction;
use super::mmio::MMIO;

/// Initialize the PL011 UART, exposing it on GPIO pins 14 and 15.
/// The UART is setup with a 115200 baudrate.
pub fn init() {
    unsafe {
        mmio::write(MMIO::UART0CR as u32, 0);
        mbox::set_uart_clock(4000000);
        enable_gpio_pin(14, GPIOFunction::Alt0);
        enable_gpio_pin(15, GPIOFunction::Alt0);
        mmio::write(MMIO::GPPUD as u32, 0);
        delay(150);
        mmio::write(MMIO::GPPUDCLK0 as u32, 1 << 14 | 1 << 15);
        delay(150);
        mmio::write(MMIO::GPPUDCLK0 as u32, 0);
        mmio::write(MMIO::UART0ICR as u32, 0x7FF);
        mmio::write(MMIO::UART0IBRD as u32, 2);
        mmio::write(MMIO::UART0FBRD as u32, 0xB);
        mmio::write(MMIO::UART0LCRH as u32, 0b11 << 5);
        mmio::write(MMIO::UART0CR as u32, 0x301);
    }
}

/// write a single character to the UART0
pub fn putc(c: u32) {
    unsafe {
        while mmio::read(MMIO::UART0FR as u32) & 0x20 != 0 {}
        mmio::write(MMIO::UART0DR as u32, c as u32);
    }
}

/// read a single character from the UART0
pub fn readc() -> u8 {
    let ch: u8;
    unsafe {
        while mmio::read(MMIO::UART0FR as u32) & 0x10 != 0 {}
        ch = mmio::read(MMIO::UART0DR as u32) as u8;
    }
    return ch;
}

/// write a string to the UART0
pub fn puts(s: &str) {
    for ch in s.chars() {
        if ch == '\n' {
            putc('\r' as u32);
        }
        putc(ch as u32);
    }
}

/// spinlock for a set number of cycles
fn delay(cycles: u32) {
    for _ in 0..cycles {
        unsafe {
            asm!("NOP");
        }
    }
}
