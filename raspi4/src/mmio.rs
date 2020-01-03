use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;

const MMIO_BASE: u32 = 0xFE000000;
const VIDEOCORE_MBOX: u32 = MMIO_BASE + 0x0000B880;

#[allow(dead_code)]
#[repr(u32)]
pub enum MMIO {
    GPFSEL = MMIO_BASE + 0x00200000,
    //GPFSEL0   = MMIO_BASE + 0x00200000,
    //GPFSEL1   = MMIO_BASE + 0x00200004,
    //GPFSEL2   = MMIO_BASE + 0x00200008,
    //GPFSEL3   = MMIO_BASE + 0x0020000c,
    //GPFSEL4   = MMIO_BASE + 0x00200010,
    //GPFSEL5   = MMIO_BASE + 0x00200014,
    GPSET0 = MMIO_BASE + 0x0020001C,
    GPSET1 = MMIO_BASE + 0x00200020,
    GPCLR0 = MMIO_BASE + 0x00200028,
    GPLEV0 = MMIO_BASE + 0x00200034,
    GPLEV1 = MMIO_BASE + 0x00200038,
    GPEDS0 = MMIO_BASE + 0x00200040,
    GPEDS1 = MMIO_BASE + 0x00200044,
    GPHEN0 = MMIO_BASE + 0x00200064,
    GPHEN1 = MMIO_BASE + 0x00200068,
    GPPUD = MMIO_BASE + 0x00200094,
    GPPUDCLK0 = MMIO_BASE + 0x00200098,
    GPPUDCLK1 = MMIO_BASE + 0x0020009C,

    UART0DR = MMIO_BASE + 0x00201000,
    UART0FR = MMIO_BASE + 0x00201018,
    UART0IBRD = MMIO_BASE + 0x00201024,
    UART0FBRD = MMIO_BASE + 0x00201028,
    UART0LCRH = MMIO_BASE + 0x0020102C,
    UART0CR = MMIO_BASE + 0x00201030,
    UART0IMSC = MMIO_BASE + 0x00201038,
    UART0ICR = MMIO_BASE + 0x00201044,

    MBOXRD = VIDEOCORE_MBOX,
    MBOXPOLL = VIDEOCORE_MBOX + 0x10,
    MBOXSNDR = VIDEOCORE_MBOX + 0x14,
    MBOXST = VIDEOCORE_MBOX + 0x18,
    MBOXCFG = VIDEOCORE_MBOX + 0x1C,
    MBOXWR = VIDEOCORE_MBOX + 0x20,
}

#[repr(u32)]
pub enum GPIOFunction {
    Input = 0b000,
    Output = 0b001,
    Alt0 = 0b100,
    Alt1 = 0b101,
    Alt2 = 0b110,
    Alt3 = 0b111,
    Alt4 = 0b011,
    Alt5 = 0b010,
}

/// # Safety
/// 
/// `reg` must be the address of a valid mmio register. This implies
/// that it has to be aligned to 4 bytes
pub unsafe fn write(reg: u32, val: u32) {
    volatile_store(reg as *mut u32, val)
}

/// # Safety
/// 
/// `reg` must be the address of a valid mmio register. This implies
/// that it has to be aligned to 4 bytes
pub unsafe fn read(reg: u32) -> u32 {
    volatile_load(reg as *const u32)
}

pub fn enable_gpio_pin(pin: u32, func: GPIOFunction) {
    let offset = pin % 10 * 3;
    let addr = MMIO::GPFSEL as u32 + (pin / 10) * 4;
    let mask = 7 << offset;
    let set = (func as u32) << offset;
    let mut val: u32;
    unsafe {
        val = read(addr);
    }
    val &= !mask;
    val |= set;
    unsafe {
        write(addr, val);
    }
}
