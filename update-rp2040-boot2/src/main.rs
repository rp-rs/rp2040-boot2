//! Compiles boot2 bootloader from assembler source
//! Compiles the bootloader from assembly language source, and creates a binary file.

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

fn make_elf<P: AsRef<Path>, Q: AsRef<Path>>(input_path: P, out_dir: Q) -> PathBuf {
    let result_path = out_dir.as_ref().join("boot2.elf");
    let output = Command::new("arm-none-eabi-gcc")
        .arg("-nostartfiles")
        .arg("-fPIC")
        .arg("--specs=nosys.specs")
        .arg(input_path.as_ref())
        .arg("-o")
        .arg(&result_path)
        .output()
        .expect("executing arm-none-eabi-gcc");
    io::stderr().write_all(&output.stderr).unwrap();
    if !output.status.success() {
        panic!("asm compile failed: {:?}", output);
    }

    result_path
}

fn make_bin<P: AsRef<Path>, Q: AsRef<Path>>(input_path: P, out_dir: Q) -> PathBuf {
    let result_path = out_dir.as_ref().join("boot2.bin");
    let output = Command::new("arm-none-eabi-objcopy")
        .arg("-O")
        .arg("binary")
        .arg(input_path.as_ref())
        .arg(&result_path)
        .output()
        .expect("executing arm-none-eabi-objcopy");
    io::stderr().write_all(&output.stderr).unwrap();
    if !output.status.success() {
        panic!(format!("asm compile failed: {:?}", output));
    }

    result_path
}

fn make_padded_bin<P: AsRef<Path>, Q: AsRef<Path>>(input_path: P, out_dir: Q) {
    const BOOT2_OUTPUT_LEN: usize = 256;
    const MAX_BOOT2_INPUT_LEN: usize = BOOT2_OUTPUT_LEN - 4;
    let mut blob = fs::read(input_path).expect("reading compiled blob for padding");
    if blob.len() >= MAX_BOOT2_INPUT_LEN {
        panic!("boot2 blob is too long!")
    }
    let num_padding_bytes = MAX_BOOT2_INPUT_LEN - blob.len();
    if num_padding_bytes > 0 {
        let padding = vec![0u8; num_padding_bytes];
        blob.extend(padding);
    }
    let crc_word = calc_crc(&blob);
    blob.extend(&crc_word.to_le_bytes());

    let result_path = out_dir.as_ref().join("boot2_padded.bin");
    fs::write(result_path, blob).expect("writing padded output file");
}

fn calc_crc(data: &[u8]) -> u32 {
    let mut engine = crc_any::CRCu32::crc32mpeg2();
    engine.digest(data);
    engine.get_crc()
}

fn main() -> Result<(), String> {
    let out_dir = "../rp2040-boot2-rs/src";

    let elf = make_elf("src/boot2_w25q080.S", &out_dir);
    let bin = make_bin(elf, &out_dir);
    let _padded_bin = make_padded_bin(bin, &out_dir);

    Ok(())
}
