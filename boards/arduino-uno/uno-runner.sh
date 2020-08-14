#!/usr/bin/env sh
set -e

if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "usage: $0 <application.elf>" >&2
    exit 1
fi

if [ "$#" -lt 1 ]; then
    echo "$0: no ELF file given" >&2
    exit 1
fi

NAME="$(basename "$1")"
SIZE_TEXT="$(avr-size "$1" | tail -1 | cut -f1)"
SIZE_DATA="$(avr-size "$1" | tail -1 | cut -f2)"
SIZE_BSS="$(avr-size "$1" | tail -1 | cut -f3)"

printf "\n"
printf "Program:             %s\n" "$NAME"
printf "Size:\n"
printf "   .text   %s (exact: %d)\n" "$(numfmt --to=si --padding=9 "$SIZE_TEXT")" "$SIZE_TEXT"
printf "   .data   %s (exact: %d)\n" "$(numfmt --to=si --padding=9 "$SIZE_DATA")" "$SIZE_DATA"
printf "   .bss    %s (exact: %d)\n" "$(numfmt --to=si --padding=9 "$SIZE_BSS")" "$SIZE_BSS"
printf "\n"
printf "Attempting to flash ...\n"
printf "\n"

avrdude -q -C/etc/avrdude.conf -patmega328p -carduino -P/dev/ttyACM0 -D "-Uflash:w:$1:e"
