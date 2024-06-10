use core::arch::asm;

pub mod serial;
pub mod vga;

unsafe fn inb(addr: u16) -> u8 {
    let mut ret: u8;
    asm!(r#"
        in al, dx 
        "#, 
        in("dx") addr,
        out("al") ret);
    ret
}

unsafe fn outb(addr: u16, value: u8) {
    asm!(r#"
        out dx, al 
        "#, 
        in("dx") addr,
        in("al") value);
}

macro_rules! print {
    ($($arg:tt)*) => {
        #[allow(unused_unsafe)]
        unsafe {
            use core::fmt::Write as FmtWrite;
            let mut writer = $crate::io::vga::TERMINAL_WRITER.borrow_mut();
            write!(writer, $($arg)*).expect("Failed to print to vga!");
        }
    }
}

macro_rules! println {
    ($($arg:tt)*) => {
        print!($($arg)*);
        print!("\n");
    }
}

macro_rules! print_serial {
    ($($arg:tt)*) => {
        #[allow(unused_unsafe)]
        unsafe {
        use core::fmt::Write as FmtWrite;
        let mut writer = $crate::io::serial::SerialPort::new();
        write!(writer, $($arg)*).expect("Failed to print to serial");
        }
    };
}

macro_rules! println_serial {
    ($($arg:tt)*) => {
        print_serial!($($arg)*);
        print_serial!("\n");
    }
}
