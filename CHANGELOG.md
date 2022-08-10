# Changelog

## Unreleased Changes

## v0.2.1

* Update boot2 code from SDK version 1.4.0
  (No changes to resulting binaries, but include structure is different)
* Add more boot2 versions:
  * BOOT_LOADER_W25X10CL
  * BOOT_LOADER_GENERIC_03H
  * BOOT_LOADER_IS25LP080

## v0.2.0

* Added AT25SF128A support
* Ensured all bootloaders are built on a `cargo build`
* Enable building without GCC by providing precompiled binaries. Use `--feature=assemble` to opt out.
* Added CI using `--feature=assemble` to verify latest boot2 source matches bootloader binary blobs

## v0.1.2

Added description to `Cargo.toml` that prevented publishing.

## v0.1.1

Fixed typo in `Cargo.toml` that prevented publishing.

## v0.1.0

Based on v1.0.0 pico boot2, but hacked to boot to a vector table, not a function, at 0x100.

