# Raspberry Pi RP2040 Bootloader

This is a second stage bootloader for the Raspberry Pi RP2040 SoC.

Include a second-stage bootloader in your application, and ensure that your linker script puts the `.boot_loader` section at the start 
of your flash image (0x000 to 0x100).

Currently only the W25Q080 flash chip (as used on the Pico) is supported.

## Licence

The assembly source is Copyright Raspberry Pi Trading and licensed under a BSD 3-clause licence. See source files for deatils.

The build.rs file is licensed as CC0.

