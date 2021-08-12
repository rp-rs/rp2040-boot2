#![no_std]

/// The bootloader to use if you have a W25Q080 flash device
pub static BOOT_LOADER_W25Q080: [u8; 256] =
    *include_bytes!(concat!(env!("OUT_DIR"), "/boot2_w25q080.padded.bin"));

/// The bootloader to use if you want to boot from RAM
pub static BOOT_LOADER_RAM_MEMCPY: [u8; 256] =
    *include_bytes!(concat!(env!("OUT_DIR"), "/boot2_ram_memcpy.padded.bin"));

/// The bootloader to use if you want to boot from an AT25SF128A flash device
pub static BOOT_LOADER_AT25SF128A: [u8; 256] =
    *include_bytes!(concat!(env!("OUT_DIR"), "/boot2_at25sf128a.padded.bin"));

#[cfg(feature = "w25q080")]
/// The 'default' boot loader is for the W25Q080 according to the Cargo features
pub static BOOT_LOADER: [u8; 256] = BOOT_LOADER_W25Q080;

#[cfg(feature = "at25sf128a")]
/// The 'default' boot loader is for the AT25SF128A according to the Cargo features
pub static BOOT_LOADER: [u8; 256] = BOOT_LOADER_AT25SF128A;

#[cfg(feature = "ram_memcpy")]
/// The 'default' boot loader is to boot from RAM according to the Cargo features
pub static BOOT_LOADER: [u8; 256] = BOOT_LOADER_RAM_MEMCPY;
