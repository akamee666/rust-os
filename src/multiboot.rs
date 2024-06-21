#[repr(C, packed)]
pub struct MultiBootInfo {
    /* Multiboot info version number */
    flags: u32,

    /* Available memory from BIOS */
    mem_lower: u32,
    mem_upper: u32,

    /* "root" partition */
    boot_device: u32,

    /* Kernel command line */
    cmdline: u32,

    /* Boot-Module list */
    mods_count: u32,
    mods_addr: u32,

    dummy: [u8; 16],

    /* Memory Mapping buffer */
    mmap_length: u32,
    mmap_addr: u32,

    /* Drive Info buffer */
    drives_length: u32,
    drives_addr: u32,

    /* ROM configuration table */
    config_table: u32,

    /* Boot Loader Name */
    boot_loader_name: *const u8,

    /* APM table */
    apm_table: u32,
}

impl MultiBootInfo {
    pub fn print_memory(&self) {
        let length = self.mmap_length;
        let addr = self.mmap_addr;
        let mut index = 0;
        while index < length {
            let m_map: *const MultibootMmapBuffer = (addr + index) as *const MultibootMmapBuffer;
            unsafe {
                let length = (*m_map).length;
                let mtype = (*m_map).mtype;
                let base = (*m_map).base_addr;
                println!(
                    "Base_addr: {:?}, Length: {:?}, Mtype: {:?}",
                    base, length, mtype
                );
            }

            index += core::mem::size_of::<MultibootMmapBuffer>() as u32;
        }
    }
}
/// 8 bytes = 64bits;
/// Each multiboot mmap entry is stored as the following:
/// 0 > size
/// 4 > base_addr_low
/// 8 > base_addr_high
/// 12 > length_low
/// 16 > length_high
/// 20 > type
#[repr(C, packed)]
struct MultibootMmapBuffer {
    size: u32,
    base_addr: u64,
    length: u64,
    mtype: u32,
}
