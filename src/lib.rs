#![no_std]

/// The bootloader to use if you have a W25Q080 flash device
pub static BOOT_LOADER_W25Q080: [u8; 256] =
    *include_bytes!(concat!(env!("OUT_DIR"), "/boot2_w25q080.padded.bin"));

/// The bootloader to use if you want to copy code to RAM and then boot from RAM
pub static BOOT_LOADER_RAM_MEMCPY: [u8; 256] =
    *include_bytes!(concat!(env!("OUT_DIR"), "/boot2_ram_memcpy.padded.bin"));

/// The bootloader to use if you want to boot from an AT25SF128A flash device
pub static BOOT_LOADER_AT25SF128A: [u8; 256] =
    *include_bytes!(concat!(env!("OUT_DIR"), "/boot2_at25sf128a.padded.bin"));

// The bootloader to use if you want to boot from an AT25SF128A flash device
pub static BOOT_LOADER_25Q64CS: [u8; 256] =
    *include_bytes!(concat!(env!("OUT_DIR"), "/boot2_25q64cs.padded.bin"));

