# Raspberry Pi RP2040 Second-Stage Bootloader

This is a second stage bootloader for the Raspberry Pi RP2040 SoC.

You can use this crate to include a second-stage bootloader in your application. Simply ensure that your linker script 
puts the array exported by this crate at the start of your flash image (0x000 to 0x100).

## Instructions

Add to your application's `Cargo.toml`:

```toml
rp2040_boot2 = { version="0.1" }
```

Add to your `main.rs`:

```rust
#[link_section = ".boot_loader"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
```

This will include support for the W25Q080 flash part on the Raspberry Pi Pico. If you have a board that uses the AT25SF128A (like the Arduino Nano Connect), you can instead use:

```rust
#[link_section = ".boot_loader"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_AT25SF128A;
```

Finally, add to your application's `memory.x`:

```
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* To suit Raspberry Pi RP2040 SoC */
  BOOT_LOADER : ORIGIN = 0x10000000, LENGTH = 0x100
  /* Adjust this to suit the size of your specific flash chip */
  FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
  RAM : ORIGIN = 0x20000000, LENGTH = 264K
}

SECTIONS {

  /* ### Boot loader */
  .boot_loader ORIGIN(BOOT_LOADER) :
  {
    KEEP(*(.boot_loader*));
  } > BOOT_LOADER

} INSERT BEFORE .text;
```

## Booting from RAM

If you want the bootloader to copy your application from flash to RAM before booting, you can use the boot loader `BOOT_LOADER_RAM_MEMCPY`, this will move all the contents from flash to RAM (up to RAM length). Using this strategy allows for faster execution and flash availability for persistent storage.

Additionally, you need to change your linker script in order to specify the VMAs & LMAs for all the RAM sections, as in this example

```
    .text : {
      ...
    } > RAM AT > FLASH
```

## Licence

Some of the assembly source files are Copyright Raspberry Pi Trading and licensed under a BSD 3-clause licence. See source files for details.

The remaining files in this crate are licensed as CC0.

