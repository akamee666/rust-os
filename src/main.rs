/* Disable warn for att_syntax, don't link the rust library and disable emitting the symbol for
   main cause we don't have the crt0 which is the entrypoint for rust binaries, our _start is at 
   boot.s so we dont have it here. */
#![allow(bad_asm_style)]
#![no_std]
#![no_main]

#[cfg(not(target_arch = "x86"))] 
compile_error!("This tutorial needs to be compiled with a x86 target");

#[macro_use]
mod vga;
mod libc;
//mod multiboot;

use core::arch::global_asm;
use core::panic::PanicInfo;
use vga::TerminalWriter;
/* Include boot.s which defines _start as inline assembly in main. This allows us to do more fine
   grained setup than if we used a naked _start function in rust. Theoretically we could use a
   naked function + some inline asm, but this seems much more straight forward. */
global_asm!(include_str!("boot.s"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]  
pub extern "C" fn kernel_main () -> ! {
    TerminalWriter::init();
    println!("something really good");
    loop{}
}

