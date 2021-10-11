#!/usr/bin/env bash

# Checks that the blobs are up to date with the committed assembly files

set -euxo pipefail

# We could just do `git diff --exit-code bin` but then all the feedback we get is
# Binary files a/bin/boot2_gd25q64cs.padded.bin and b/bin/boot2_gd25q64cs.padded.bin differ
# so lets dissassemble with objdump to get better feedback

git checkout bin
git clean -f bin

cargo clean

for lib in bin/*.bin; do
    filename=$(basename "$lib")
    arm-none-eabi-objdump -b binary -m armv6-m -M force-thumb -D "$lib" > "bin/${filename%.bin}.before"
done

cargo build --features=assemble

for lib in bin/*.bin; do
    filename=$(basename "$lib")
    arm-none-eabi-objdump -b binary -m armv6-m -M force-thumb -D "$lib" > "bin/${filename%.bin}.after"
done

for disassembly in bin/*.after; do
    diff -u "$disassembly" "${disassembly%.after}.before"
done
