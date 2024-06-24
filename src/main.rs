#![no_std]
#![no_main]
#![allow(named_asm_labels)]

#[macro_use]
mod io;
mod gdt;
mod multiboot;
mod util;

use crate::io::serial::SerialPort;
use crate::multiboot::MultiBootInfo;
use core::arch::global_asm;
use core::panic::PanicInfo;
use io::vga::TerminalWriter;

/* Include boot.s which defines _start as inline assembly in main. This allows us to do more fine
grained setup than if we used a naked _start function in rust. Theoretically we could use a
naked function + some inline asm, but this seems much more straight forward. */
global_asm!(include_str!("boot.s"), options(att_syntax));

pub fn test_runner(tests: &[&dyn Fn()]) {
    // tests = list of references of types that can be called like an function(&dyn).
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn kernel_main(info: *const MultiBootInfo) -> i32 {
    TerminalWriter::init();
    SerialPort::init().expect("Failed to init SerialPort");
    gdt::init();

    println!("Calling print_gdt in main");
    gdt::print_gdt();
    println!("");
    multiboot::MultiBootInfo::print_memory(&(*info));
    io::exit(0);
    0
}
