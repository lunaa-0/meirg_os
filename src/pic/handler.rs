use super::ports::{inb, outb};
use crate::vga_print;


#[repr(C)]
pub struct InterruptStackFrame {
    pub instruction_pointer: u64,
    pub code_segements: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segments: u64,
}

static mut KEYBOARD_BUFFER: [u8; 256] = [0; 256];
static mut KEYBOARD_BUFFER_HEAD: usize = 0;

pub extern "x86-interrupt" fn keyboard_handler(_stack_frame: &mut InterruptStackFrame) {
    let scancode: u8;
    scancode = inb(0x60);
    let ascii_input: Option<u8> = scancode_to_ascii(scancode);
    if let Some(ascii) = ascii_input {
        
        vga_print::write_byte(ascii);
        /*unsafe {
            KEYBOARD_BUFFER[KEYBOARD_BUFFER_HEAD % 256] = ascii;
            KEYBOARD_BUFFER_HEAD += 1;
        }*/
    }
    outb(0x20, 0x20);
}


#[inline(always)]
pub fn write_from_keyboard_buffer() {
    unsafe {
        while KEYBOARD_BUFFER_HEAD > 0 {
            let byte = KEYBOARD_BUFFER[0];
            for i in 1..KEYBOARD_BUFFER_HEAD {
                KEYBOARD_BUFFER[i - 1] = KEYBOARD_BUFFER[i];
            }
            KEYBOARD_BUFFER_HEAD -= 1;
            vga_print::write_byte(byte);
        }
    }
} 


pub fn scancode_to_ascii(scancode: u8) -> Option<u8> {
    match scancode {
        1 => Some(27),
        2 => Some(b'1'),
        3 => Some(b'2'),
        4 => Some(b'3'),
        5 => Some(b'4'),
        6 => Some(b'5'),
        7 => Some(b'6'),
        8 => Some(b'7'),
        9 => Some(b'8'),
        10 => Some(b'9'),
        11 => Some(b'0'),
        12 => Some(b'-'),
        13 => Some(b'='),
        14 => Some(8),
        15 => Some(9), //Backspace
        16 => Some(b'q'),
        17 => Some(b'w'),
        18 => Some(b'e'),
        19 => Some(b'r'),
        20 => Some(b't'),
        21 => Some(b'y'),
        22 => Some(b'u'),
        23 => Some(b'i'),
        24 => Some(b'o'),
        25 => Some(b'p'),
        26 => Some(b'['),
        27 => Some(b']'),
        28 => Some(13), // Enter (CR)
        30 => Some(b'a'),
        31 => Some(b's'),
        32 => Some(b'd'),
        33 => Some(b'f'),
        34 => Some(b'g'),
        35 => Some(b'h'),
        36 => Some(b'j'),
        37 => Some(b'k'),
        38 => Some(b'l'),
        39 => Some(b';'),
        40 => Some(b'\''),
        41 => Some(b'`'),
        43 => Some(b'\\'),
        44 => Some(b'z'),
        45 => Some(b'x'),
        46 => Some(b'c'),
        47 => Some(b'v'),
        48 => Some(b'b'),
        49 => Some(b'n'),
        50 => Some(b'm'),
        51 => Some(b','),
        52 => Some(b'.'),
        53 => Some(b'/'),
        55 => Some(b'*'),
        57 => Some(b' '),
        74 => Some(b'-'),
        78 => Some(b'+'),
        102 => Some(8),//Backspace
        _ => None,
    }
}
