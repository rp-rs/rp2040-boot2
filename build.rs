//! Compiles boot2 bootloader from assembler source
//! Compiles the bootloader from assembly language source, and creates a binary file.

use std::env;
use std::fs;
#[cfg(feature = "assemble")]
use std::io::{self, Write};
use std::path::Path;
#[cfg(feature = "assemble")]
use std::path::PathBuf;
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
        .arg("-DPICO_FLASH_SPI_CLKDIV=2")
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
fn make_padded_bin<P: AsRef<Path>, Q: AsRef<Path>>(input_path: P, out_dir: Q) -> PathBuf {
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
    fs::write(&result_path, blob).expect("writing padded output file");
    result_path
}

#[cfg(feature = "assemble")]
fn calc_crc(data: &[u8]) -> u32 {
    let mut engine = crc_any::CRCu32::crc32mpeg2();
    engine.digest(data);
    engine.get_crc()
}

#[cfg(feature = "assemble")]
fn update_precompiled_bin<P: AsRef<Path>>(input_path: P) {
    let input_path: &Path = input_path.as_ref();
    // Abort if this crate is being built as a dependency.
    // This check is crude, but CARGO_PRIMARY_PACKAGE is not
    // available in build scripts.
    if !env::var("OUT_DIR")
        .unwrap()
        .starts_with(&env::var("CARGO_MANIFEST_DIR").unwrap())
    {
        panic!(
            "UPDATE_PRECOMPILED_BINARIES must only be used when compiling this package directly"
        );
    }
    let precompiled_bin_dir = env::var("CARGO_MANIFEST_DIR").unwrap() + "/bin/";
    std::fs::copy(
        &input_path,
        Path::new(&precompiled_bin_dir).join(input_path.file_name().unwrap()),
    )
    .unwrap();
}

#[cfg(feature = "assemble")]
fn main() -> Result<(), String> {
    let out_dir = env::var("OUT_DIR").unwrap();
    for asm_file in SOURCE_FILES.iter() {
        let elf = make_elf(asm_file, &out_dir);
        let bin = make_bin(elf, &out_dir);
        let padded_bin = make_padded_bin(bin, &out_dir);
        if env::var("UPDATE_PRECOMPILED_BINARIES").is_ok() {
            update_precompiled_bin(padded_bin);
        }
        println!("cargo:rerun-if-changed={}", asm_file);
    }
    println!("cargo:rerun-if-changed=./build.rs");
    println!("cargo:rerun-if-env-changed=UPDATE_PRECOMPILED_BINARIES");

    Ok(())
}

#[cfg(not(feature = "assemble"))]
fn main() -> Result<(), String> {
    let in_dir = env::var("CARGO_MANIFEST_DIR").unwrap() + "/bin/";
    let out_dir = env::var("OUT_DIR").unwrap();

    let paths: Vec<_> = fs::read_dir(in_dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    for path in paths {
        if path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .ends_with(".padded.bin")
        {
            std::fs::copy(&path, Path::new(&out_dir).join(path.file_name().unwrap())).unwrap();
        }
    }
    println!("cargo:warning=Using prebuilt boot2 files. use feature `assemble` to rebuild instead (requires GNU toolchain)");
    Ok(())
}
