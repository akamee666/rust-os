use core::{cell::RefCell, fmt::Write};

// Required method from core::fmt::Write;
impl Write for TerminalWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s.as_bytes());
        Ok(())
    }
}

// Create a new instance of RefCell where the value is an TerminalWriter struct.
pub static TERMINAL_WRITER: StaticTerminalWriter = StaticTerminalWriter::new();

pub struct StaticTerminalWriter {
    inner: RefCell<TerminalWriter>,
}

impl StaticTerminalWriter {
    const fn new() -> StaticTerminalWriter {
        StaticTerminalWriter {
            inner: RefCell::new(TerminalWriter::new()),
        }
    }
}

// https://timclicks.dev/explaining-rusts-deref-trait/
// Whenever i use StaticTerminalWriter, deref method will be used to get the inner value(The real
// struct actually)
impl core::ops::Deref for StaticTerminalWriter {
    type Target = RefCell<TerminalWriter>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

unsafe impl Sync for StaticTerminalWriter {}

#[allow(dead_code)]
enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    LightBrown = 14,
    White = 15,
}

const fn vga_entry_color(fg: VgaColor, bg: VgaColor) -> u8 {
    fg as u8 | (bg as u8) << 4
}

const fn vga_entry(uc: u8, color: u8) -> u16 {
    uc as u16 | (color as u16) << 8
}

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

// Put row and column together may help the redibility for my code.
pub struct TerminalWriter {
    terminal_row: usize,
    terminal_column: usize,
    terminal_color: u8,
    terminal_buffer: *mut u16,
}

impl TerminalWriter {
    // the constructor, will be used once. After that, we'll use init to get a pointer to the
    // global TERMINAL_WRITER struct once initializated from new()
    const fn new() -> TerminalWriter {
        let terminal_row = 0;
        let terminal_column = 0;
        let terminal_color = vga_entry_color(VgaColor::LightGrey, VgaColor::Black);
        let terminal_buffer = 0xB8000 as *mut u16;

        /* Return the initialized values from the function above*/
        TerminalWriter {
            terminal_row,
            terminal_column,
            terminal_color,
            terminal_buffer,
        }
    }

    pub fn init() {
        let terminal = TERMINAL_WRITER.borrow_mut();
        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                // multiples the current row by the total number of chars per row
                // and add the current column after that.
                let index = y * VGA_WIDTH + x;
                unsafe {
                    *terminal.terminal_buffer.add(index) = vga_entry(b' ', terminal.terminal_color);
                }
            }
        }
    }

    #[allow(dead_code)]
    fn set_color(&mut self, color: u8) {
        self.terminal_color = color;
    }

    fn put_entry_at(&mut self, character: u8, color: u8, x: usize, y: usize) {
        let index = y * VGA_WIDTH + x;
        unsafe {
            *self.terminal_buffer.add(index) = vga_entry(character, color);
        }
    }

    fn putchar(&mut self, character: u8) {
        if character == b'\n' {
            self.terminal_row += 1;
            self.terminal_column = 0;
            return;
        }
        self.put_entry_at(
            character,
            self.terminal_color,
            self.terminal_column,
            self.terminal_row,
        );
        self.terminal_column += 1;
        if self.terminal_column == VGA_WIDTH {
            self.terminal_column = 0;
            self.terminal_row += 1;
            if self.terminal_row == VGA_HEIGHT {
                self.terminal_row = 0;
            }
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        for character in data {
            self.putchar(*character);
        }
    }
}
