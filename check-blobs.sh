#!/usr/bin/env bash

# Checks that the blobs are up to date with the committed assembly files

set -euxo pipefail

for lib in bin/*.bin; do
    filename=$(basename "$lib")
    arm-none-eabi-objdump -b binary -m armv6s-m -M force-thumb -D "$lib" > "bin/${filename%.bin}.before"
done

cargo build --release
# ./assemble.sh

for lib in bin/*.bin; do
    filename=$(basename "$lib")
    arm-none-eabi-objdump -b binary -m armv6s-m -M force-thumb -D > "bin/${filename%.bin}.after"
done

for cksum in bin/*.after; do
    diff -u "$cksum" "${cksum%.after}.before"
done