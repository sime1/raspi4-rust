use core::intrinsics::transmute;

use super::mmio;
use super::mmio::MMIO;

use macros::mailbox_request;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum MailboxStatus {
    Full = 0x80000000,
    Empty = 0x40000000,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum MailboxCode {
    Request = 0x0,
    ResponseSuccess = 0x80000000,
    ResponseError = 0x80000001,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum MailboxTag {
    SetClock = 0x00038002,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum MailboxChannel {
    Power = 0,
    Framebuffer = 1,
    VUART = 2,
    VCHIQ = 3,
    Led = 4,
    Button = 5,
    TouchScreen = 6,
    Tags = 7,
    ARM = 8,
}

#[repr(u32)]
pub enum ClockId {
    EMMC = 1,
    UART,
    ARM,
    CORE,
    V3D,
    // TODO: add all the IDs
}

#[repr(C)]
struct MailboxRequestHeader {
    buffer_size: u32,
    code: MailboxCode,
    tag_id: MailboxTag,
    tag_size: u32,
    unknown: u32,
}

impl Clone for MailboxRequestHeader {
    fn clone(&self) -> Self {
        Self {
            buffer_size: self.buffer_size,
            code: self.code,
            tag_id: self.tag_id,
            tag_size: self.tag_size,
            unknown: self.unknown,
        }
    }
}

#[mailbox_request(36, MailboxCode::Request, MailboxTag::SetClock, 12)]
struct SetClockRateRequest {
    id: ClockId,
    rate: u32,
    skip_turbo: u32,
}

impl Default for SetClockRateRequest {
    fn default() -> Self {
        Self {
            header: SetClockRateRequestHeader,
            id: ClockId::ARM,
            rate: 4000000,
            skip_turbo: 0,
            end_tag: 0,
        }
    }
}

pub unsafe fn call_mbox(buf: *const u32, ch: MailboxChannel) {
    let r = (transmute::<*const u32, u64>(buf) as u32 & !0xF) | (ch as u32 & 0xF);
    let mbox_status = MMIO::MBOXST as u32;
    let mbox_write = MMIO::MBOXWR as u32;
    wait_status(mbox_status, MailboxStatus::Full);
    mmio::write(mbox_write, r);
    loop {
        wait_status(mbox_status, MailboxStatus::Empty);
        let res = mmio::read(MMIO::MBOXRD as u32);
        if r == res {
            return;
        }
    }
}

unsafe fn wait_status(mem: u32, target: MailboxStatus) {
    let t = target as u32;
    while (mmio::read(mem) & t) != 0 {}
}

pub fn set_uart_clock(rate: u32) {
    let req = SetClockRateRequest {
        id: ClockId::UART,
        rate: rate,
        ..Default::default()
    };
    unsafe {
        asm!("dsb sy");
        call_mbox(
            &req as *const SetClockRateRequest as *const u32,
            MailboxChannel::ARM,
        );
    }
}
