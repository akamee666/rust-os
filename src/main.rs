/* Disable warn for att_syntax, don't link the rust library and disable emitting the symbol for
main cause we don't have the crt0 which is the entrypoint for rust binaries, our _start is at
boot.s so we dont have it here.
To fix this, we first need to change the name of the generated function to something different than main through the reexport_test_harness_main attribute. Then we can call the renamed function from our _start function:
*/
#![allow(bad_asm_style)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![no_std]
#![no_main]

#[macro_use]
mod io;
mod libc;

use core::arch::global_asm;
use core::panic::PanicInfo;
use io::vga::TerminalWriter;

use crate::io::serial::SerialPort;

/* Include boot.s which defines _start as inline assembly in main. This allows us to do more fine
grained setup than if we used a naked _start function in rust. Theoretically we could use a
naked function + some inline asm, but this seems much more straight forward. */
global_asm!(include_str!("boot.s"));

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
pub unsafe extern "C" fn kernel_main() -> ! {
    // Clear Terminal, not used to print anymore!
    TerminalWriter::init();
    SerialPort::init().expect("Failed to init SerialPort");
    #[cfg(test)]
    test_main();

    println!("Hello kernel world!");
    println_serial!("Hello Kernel but now in stdio");
    loop {}
}

#[cfg(test)]
mod tests {}
