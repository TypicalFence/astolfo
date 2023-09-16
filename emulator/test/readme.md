# Emulator Tests

These tests consist of tiny assembly programs, ideally they only use one instruction.

To ensure opcodes are parsed correctly they should be assembled by the avr-gcc toolchain.

## asm to bin
```sh
avr-as -mmcu=atmega328p test.A
avr-objcopy -O binary -R .eeprom a.out test.bin
```