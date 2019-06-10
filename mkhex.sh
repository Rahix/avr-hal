#!/bin/sh
if [ $# -gt 2 ]; then
    echo "usage: $0 <elf-name>"
    exit 1
fi

BUILD="debug"
if [ "$1" = "--debug" ]; then
    shift 1
    BUILD="debug"
elif [ "$1" = "--release" ]; then
    shift 1
    BUILD="release"
fi

CWD="$(pwd)"
ELF="$(realpath --relative-to="$CWD" "$(dirname "$0")"/target/avr-*/$BUILD/examples/"$1.elf")"
HEX="$(realpath --relative-to="$CWD" "$(dirname "$0")"/target/"$1.hex")"

set -xe

avr-objcopy -S -j .text -j .data -O ihex "$ELF" "$HEX"

set +x

BYTES=$(avr-size "$ELF" | tail -1 | cut -f4 | bc)
echo "$(numfmt --to=si "$BYTES") Bytes used ($BYTES exact)."
