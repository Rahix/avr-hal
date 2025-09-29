# Flash attiny402 with UPDI

FILE=../../target/avr-none/release/avrmodern-blink.elf
#FILE=../../target/avr-none/release/avrmodern-usart.elf

objcopy -O ihex $FILE output.hex
"$HOME/.arduino15/packages/megaTinyCore/tools/python3/3.7.2-post1/python3" -u "$HOME/.arduino15/packages/megaTinyCore/hardware/megaavr/2.6.10/tools/prog.py"  -t uart -u /dev/ttyUSB0 -b 230400 -d attiny402 --fuses 0:0b00000000 2:0x01 6:0x04 7:0x00 8:0x00 "-foutput.hex" -a write -v
