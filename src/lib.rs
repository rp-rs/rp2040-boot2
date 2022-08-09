#![no_std]

/// The bootloader to use if you have a W25Q080 flash device
pub static BOOT_LOADER_W25Q080: [u8; 256] =
    *include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/bin/boot2_w25q080.padded.bin"));

/// The bootloader to use if you want to copy code to RAM and then boot from RAM
pub static BOOT_LOADER_RAM_MEMCPY: [u8; 256] =
    *include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/bin/boot2_ram_memcpy.padded.bin"));

/// The bootloader to use if you want to boot from an AT25SF128A flash device
pub static BOOT_LOADER_AT25SF128A: [u8; 256] =
    *include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/bin/boot2_at25sf128a.padded.bin"));

/// The bootloader to use if you want to boot from an GD25Q64CS flash device
pub static BOOT_LOADER_GD25Q64CS: [u8; 256] =
    *include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/bin/boot2_gd25q64cs.padded.bin"));

/// The bootloader to use if you want to boot from an W25X10CL flash device
pub static BOOT_LOADER_W25X10CL: [u8; 256] =
    *include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/bin/boot2_w25x10cl.padded.bin"));

/// The bootloader to use if you want to boot from a generic flash device
pub static BOOT_LOADER_GENERIC_03H: [u8; 256] =
    *include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/bin/boot2_generic_03h.padded.bin"));

/// The bootloader to use if you want to boot from an IS25LP080 flash device
pub static BOOT_LOADER_IS25LP080: [u8; 256] =
    *include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/bin/boot2_is25lp080.padded.bin"));
