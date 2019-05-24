#!/bin/sh
if [ $# -ne 1 ]; then
    echo "usage: $0 <elf-name>"
    exit 1
fi

# Go to the project root
cd "$(dirname "$0")"

ELF="$(echo target/avr-*/release/examples/"$1.elf")"

set -xe

avr-objcopy -S -j .text -j .data -O ihex "$ELF" "target/$1.hex"

set +x

BYTES=$(avr-size "$ELF" | tail -1 | cut -f4 | bc)
echo "$(numfmt --to=si "$BYTES") Bytes used ($BYTES exact)."
