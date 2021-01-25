# Raspberry Pi RP2040 Second-Stage Bootloader

This is a second stage bootloader for the Raspberry Pi RP2040 SoC. Currently only the W25Q080 flash chip (as used on 
the Pico) is supported.

You can use this crate to include a second-stage bootloader in your application. Simply ensure that your linker script 
puts the array exported by this crate at the start of your flash image (0x000 to 0x100).

## Instructions

Add to your `main.rs`:

```rust
#[link_section = ".boot_loader"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER;
```

Add to your application's `Cargo.toml`:

```toml
rp2040_boot2 = "*" # Latest from crates.io, or do
rp2040_boot2 = { git = "https://github.com/rp-rs/rp2040_boot2", branch="main" } # Latest from github
```

And add to your application's `memory.x`:

```
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* To suit Raspberry Pi RP2040 SoC */
  BOOT_LOADER : ORIGIN = 0x10000000, LENGTH = 0x100
  FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
  RAM : ORIGIN = 0x20000000, LENGTH = 256K
}

SECTIONS {

  /* ### Boot loader */
  .boot_loader ORIGIN(BOOT_LOADER) :
  {
    KEEP(*(.boot_loader*));
  } > BOOT_LOADER

} INSERT BEFORE .text;
```

## Licence

The assembly source is Copyright Raspberry Pi Trading and licensed under a BSD 3-clause licence. See source files for details.

The build.rs file is licensed as CC0.

