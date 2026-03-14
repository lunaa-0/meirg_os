#![allow(dead_code)]
use core::ptr;

use super::ports::{inb, outb};

const PIC1_COMMAND:u16 = 0x20;
const PIC2_COMMAND:u16 = 0xA0;
const PIC1_DATA:u16=0x21;
const PIC2_DATA:u16=0xA1;

///remaps the pic so it doesn't collide with other cpu interrupts
pub fn remap_pic(){
    outb(PIC1_COMMAND, 0x11);//restart master pic
    outb(PIC2_COMMAND, 0x11);//restart slave pic

    //set offset
    outb(PIC1_DATA, 0x20);
    outb(PIC2_DATA, 0x28);

    //setup master/slave relationship
    outb(PIC1_DATA, 4);
    outb(PIC2_DATA, 2);

    //set 8086 mode
    outb(PIC1_DATA, 0x01);
    outb(PIC2_DATA, 0x01);

    //disable all irqs except for keyboard
    outb(PIC1_DATA, 0xFD);
    outb(PIC2_DATA, 0xFF); 
}

