#![no_std]

pub static BOOT_LOADER: [u8; 256] = *include_bytes!("boot2_padded.bin");
