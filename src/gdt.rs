use core::arch::{asm, global_asm};

#[repr(C, packed)]
struct Gdt {
    limit: u16,
    base: u32,
}

fn read_gdt_reg() -> u64 {
    let mut gdtr = 0u64;
    unsafe {
        asm!(r#"
        sgdt ({})
        "#, 
        in (reg) &mut gdtr, options(att_syntax));
        gdtr
    }
}

fn create_descriptor(base: u32, limit: u32, access_bits: u16, flags: u8) -> u64 {
    // LIMIT 0:15, 48:51; 0xFFFFF
    // BASE  16:31, 32:39, 56:63; 0x00000000
    // FlAGS 52:55, 0b0011
    // limit and base, flags and access bits right here!.
    let mut descriptor = 0u64;

    descriptor |= (limit & 0x00FFFF) as u64; // 0:15
    println!("{:#x}", descriptor);
    descriptor |= ((limit & 0x0000000F) as u64) << 48; // 48:51
    println!("second limit {:#x}", descriptor);
    descriptor |= ((base << 16) & 0x0000FFFF) as u64; // 16:31
    descriptor |= ((base & 0xFFFFF00) as u64) << 32; // 32:40
    descriptor |= ((base & 0xFFFFF00) as u64) << 56; // 56:63
    descriptor |= ((flags & 0x0F) as u64) << 52; // 52:55
    descriptor |= ((access_bits & 0xFFFF) as u64) << 40; // 40:48
    println!("{:#x}", descriptor);
    descriptor
}

pub fn init() {
    create_descriptor(0, 0xfffffff, 0b11011001, 0b1100);
}
