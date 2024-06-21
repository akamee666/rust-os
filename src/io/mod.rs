use core::arch::asm;

pub mod serial;
pub mod vga;

unsafe fn inb(addr: u16) -> u8 {
    let mut ret: u8;
    asm!(r#"
        in %dx, %al 
        "#, 
        in("dx") addr,
        out("al") ret, options(att_syntax));
    ret
}

unsafe fn outb(addr: u16, value: u8) {
    asm!(r#"
        out %al, %dx
        "#, 
        in("dx") addr,
        in("al") value, options(att_syntax));
}

pub unsafe fn exit(code: u8) {
    let isa_debug_exit_port: u16 = 0xf4;
    outb(isa_debug_exit_port, code);
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
        print_serial!($($arg)*);
        print_serial!("\n");
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
