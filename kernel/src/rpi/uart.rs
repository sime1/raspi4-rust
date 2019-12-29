use super::mbox;
use super::rpi::enable_gpio_pin;
use super::rpi::mmio_read;
use super::rpi::mmio_write;
use super::rpi::GPIOFunction;
use super::rpi::MMIO;

pub fn init() {
    unsafe {
        mmio_write(MMIO::UART0CR as u32, 0);
        mbox::set_uart_clock(4000000);
        enable_gpio_pin(14, GPIOFunction::Alt0);
        enable_gpio_pin(15, GPIOFunction::Alt0);
        mmio_write(MMIO::GPPUD as u32, 0);
        delay(150);
        mmio_write(MMIO::GPPUDCLK0 as u32, 1 << 14 | 1 << 15);
        delay(150);
        mmio_write(MMIO::GPPUDCLK0 as u32, 0);
        mmio_write(MMIO::UART0ICR as u32, 0x7FF);
        mmio_write(MMIO::UART0IBRD as u32, 2);
        mmio_write(MMIO::UART0FBRD as u32, 0xB);
        mmio_write(MMIO::UART0LCRH as u32, 0b11 << 5);
        mmio_write(MMIO::UART0CR as u32, 0x301);
    }
}

pub fn putc(c: u32) {
    unsafe {
        while mmio_read(MMIO::UART0FR as u32) & 0x20 != 0 {}
        mmio_write(MMIO::UART0DR as u32, c);
    }
}

pub fn readc() -> u8 {
    let ch: u8;
    unsafe {
        while mmio_read(MMIO::UART0FR as u32) & 0x10 != 0 {}
        ch = mmio_read(MMIO::UART0DR as u32) as u8;
    }
    return ch;
}

pub fn puts(s: &str) {
    for ch in s.chars() {
        putc(ch as u32)
    }
}

fn delay(cycles: u32) {
    for _ in 0..cycles {
        unsafe {
            asm!("NOP");
        }
    }
}
