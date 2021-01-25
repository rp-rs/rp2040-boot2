#![no_std]

pub static BOOT_LOADER: [u8; 256] = *include_bytes!(concat!(env!("OUT_DIR"), "/boot2_padded.bin"));
