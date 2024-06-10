use crate::io::{inb, outb};

const PORT: u16 = 0x3f8; // COM BASE ADDRESS!

#[derive(Debug)]
pub struct SerialInitError;

pub struct SerialPort {}

impl SerialPort {
    pub fn new() -> SerialPort {
        SerialPort {}
    }

    pub unsafe fn init() -> Result<(), SerialInitError> {
        outb(PORT + 1, 0x00); // Disable all interrupts
        outb(PORT + 3, 0x80); // Enable DLAB (set baud rate divisor)
        outb(PORT + 0, 0x03); // Set divisor to 3 (lo byte) 38400 baud
        outb(PORT + 1, 0x00); //                  (hi byte)
        outb(PORT + 3, 0x03); // 8 bits, no parity, one stop bit
        outb(PORT + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
        outb(PORT + 4, 0x0B); // IRQs enabled, RTS/DSR set
        outb(PORT + 4, 0x1E); // Set in loopback mode, test the serial chip
        outb(PORT + 0, 0xAE); // Test serial chip (send byte 0xAE and check if serial returns same byte)

        // Check if serial is faulty (i.e: not same byte as sent)
        if inb(PORT + 0) != 0xAE {
            return Err(SerialInitError);
        }

        // If serial is not faulty set it in normal operation mode
        // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
        outb(PORT + 4, 0x0F);

        Ok(())
    }
}

unsafe fn write_serial(c: u8) {
    while is_transmit_empty() == 0 {}
    outb(PORT, c);
}

unsafe fn is_transmit_empty() -> u8 {
    return inb(PORT + 5) & 0x20;
}

impl core::fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            for b in s.as_bytes() {
                write_serial(*b);
            }
        }
        Ok(())
    }
}
