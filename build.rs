//! Compiles boot2 bootloader from assembler source
//! Compiles the bootloader from assembly language source, and creates a binary file.

#[cfg(feature = "assemble")]
use std::env;
#[cfg(feature = "assemble")]
use std::fs;
#[cfg(feature = "assemble")]
use std::io::{self, Write};
#[cfg(feature = "assemble")]
use std::path::{Path, PathBuf};
#[cfg(feature = "assemble")]
use std::process::Command;

#[cfg(feature = "assemble")]
static SOURCE_FILES: &[&'static str] = &[
    "src/boot2_at25sf128a.S",
    "src/boot2_ram_memcpy.S",
    "src/boot2_w25q080.S",
    "src/boot2_gd25q64cs.S",
    "src/boot2_w25x10cl.S",
    "src/boot2_generic_03h.S",
    "src/boot2_is25lp080.S",
];

#[cfg(feature = "assemble")]
fn make_elf<P: AsRef<Path>, Q: AsRef<Path>>(input_path: P, out_dir: Q) -> PathBuf {
    let input_path: &Path = input_path.as_ref();
    let mut result_file = PathBuf::from(input_path.file_name().unwrap());
    result_file.set_extension("elf");
    let result_path = out_dir.as_ref().join(result_file);
    let output = Command::new("arm-none-eabi-gcc")
        .arg("-nostartfiles")
        .arg("-fPIC")
        .arg("--specs=nosys.specs")
        .arg("-Isrc/include")
        .arg(input_path)
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

#[cfg(feature = "assemble")]
fn make_bin<P: AsRef<Path>, Q: AsRef<Path>>(input_path: P, out_dir: Q) -> PathBuf {
    let input_path: &Path = input_path.as_ref();
    let mut result_file = PathBuf::from(input_path.file_name().unwrap());
    result_file.set_extension("bin");
    let result_path = out_dir.as_ref().join(result_file);
    let output = Command::new("arm-none-eabi-objcopy")
        .arg("-O")
        .arg("binary")
        .arg(input_path)
        .arg(&result_path)
        .output()
        .expect("executing arm-none-eabi-objcopy");
    io::stderr().write_all(&output.stderr).unwrap();
    if !output.status.success() {
        panic!("asm compile failed: {:?}", output);
    }

    result_path
}

#[cfg(feature = "assemble")]
fn make_padded_bin<P: AsRef<Path>, Q: AsRef<Path>>(input_path: P, out_dir: Q) {
    const BOOT2_OUTPUT_LEN: usize = 256;
    const MAX_BOOT2_INPUT_LEN: usize = BOOT2_OUTPUT_LEN - 4;
    let input_path: &Path = input_path.as_ref();
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

    let mut result_file = PathBuf::from(input_path.file_name().unwrap());
    result_file.set_extension("padded.bin");
    let result_path = out_dir.as_ref().join(result_file);
    fs::write(result_path, blob).expect("writing padded output file");
}

#[cfg(feature = "assemble")]
fn calc_crc(data: &[u8]) -> u32 {
    let mut engine = crc_any::CRCu32::crc32mpeg2();
    engine.digest(data);
    engine.get_crc()
}

#[cfg(feature = "assemble")]
fn main() -> Result<(), String> {
    // Store temporary build files here
    let out_dir = env::var("OUT_DIR").unwrap();
    // And our final output here
    let final_outdir = concat!(env!("CARGO_MANIFEST_DIR"), "/bin/");
    for asm_file in SOURCE_FILES.iter() {
        let elf = make_elf(asm_file, &out_dir);
        let bin = make_bin(elf, &out_dir);
        let _padded_bin = make_padded_bin(bin, &final_outdir);
        println!("cargo:rerun-if-changed={}", asm_file);
    }
    println!("cargo:rerun-if-changed=./build.rs");

    Ok(())
}

#[cfg(not(feature = "assemble"))]
fn main() -> Result<(), String> {
    println!("cargo:warning=Using prebuilt boot2 files. use feature `assemble` to rebuild instead (requires GNU toolchain)");
    Ok(())
}
