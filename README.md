# HAL_esilv

## Pre-processing, compiling and assembling
cargo +nightly build -Zbuild-std=core --target avr-unknown-gnu-atmega328 --release
--> error linking avr-gcc ...

## Linking
C:\WinAVR-20100110\bin\avr-gcc.exe -mmcu=atmega328 -o output.elf .\target\avr-unknown-gnu-atmega328\release\deps\*.o .\target\avr-unknown-gnu-atmega328\release\deps\*.rlib

## Convert .elf to .hex
C:\WinAVR-20100110\bin\avr-objcopy.exe -O ihex .\output.elf output.hex 

## Board flashing (cf AVRDUDE)
C:\WinAVR-20100110\bin\avrdude.exe -C C:\WinAVR-20100110\bin\avrdude.conf -v -patmega328p -carduino -PCOM6 -b115200 -Uflash:w:output.hex:i
PCOM6 (depending on what port your atmega is)