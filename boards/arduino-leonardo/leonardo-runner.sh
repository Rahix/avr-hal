#!/usr/bin/env sh
set -e

case "$(uname -s)" in
    Linux*)     OS="Linux";;
    Darwin*)    OS="Mac";;
    *)          OS="Unknown";;
esac

if ! command -v numfmt &> /dev/null
then
    echo "numfmt is needed for human-readable sizes." >&2
    echo "please install https://command-not-found.com/numfmt" >&2
    alias numfmt=true
fi

if ! command -v avrdude &> /dev/null
then
    echo "required avrdude could not be found!" >&2
    echo "please install https://command-not-found.com/avrdude" >&2
    exit 1
fi

if [ $OS = "Linux" ]; then
    SERIAL_PORT="/dev/ttyACM0"
elif [ $OS = "Mac" ]; then
    SERIAL_PORT="/dev/cu.usbmodem146201"
else
    echo "unsupported OS, things might not work" >&2
    SERIAL_PORT="/dev/ttyACM0"
fi

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
printf "Please bring up the bootloader and press ENTER!\n"
read -r
printf "Attempting to flash ...\n"
printf "\n"

avrdude -qq -patmega32u4 -cavr109 -P"${SERIAL_PORT}" -b57600 -D "-Uflash:w:$1:e"
