#![allow(dead_code)]

use core::ptr;

use crate::pic::{handler::keyboard_handler, remap::remap_pic};

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct IdtEntry {
    offset_1: u16,
    selector: u16,
    ist: u8,
    type_attributes: u8,
    offset_2: u16,
    offset_3: u32,
    zero: u32,
}

impl IdtEntry {
    pub fn new() -> Self {
        IdtEntry {
            offset_1: 0,
            selector: 0x08,
            ist: 0,
            type_attributes: 0b1000_1110,
            offset_2: 0,
            offset_3: 0,
            zero: 0,
        }
    }

    pub fn set_handler(&mut self, handler: u64) {
        self.offset_1 = handler as u16;
        self.offset_2 = (handler >> 16) as u16;
        self.offset_3 = (handler >> 32) as u32;
    }
}

static mut IDT: [IdtEntry; 256] = [IdtEntry {
    offset_1: 0,
    selector: 0x08,
    ist: 0,
    type_attributes: 0b1000_1110, //P=true, DPL=Ring 0, S=false, Interrupt Gate
    offset_2: 0,
    offset_3: 0,
    zero: 0,
}; 256];

#[repr(C, packed)]
struct IDTDescriptor {
    limit: u16,
    base: u64,
}

pub fn init_idt() {
    unsafe {
        core::arch::asm!("cli");

        remap_pic();

        IDT[33].set_handler(keyboard_handler as u64);

        let descriptor = IDTDescriptor {
            limit: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
            base: &raw const IDT as *const _ as u64,
        };

        core::arch::asm!(
            "lidt [{}]",
            in(reg) &descriptor
        );
        core::arch::asm!("sti")
    }
}
