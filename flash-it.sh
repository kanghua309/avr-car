# prevent flashing if error
set -e cargo +nightly build

avrdude -p atmega328p -c arduino -P /dev/tty.usbserial-A50285BI -U flash:w:target/avr-atmega328p/debug/avr-robot-car.elf:e
# show on console with baud rate 57600
screen /dev/tty.usbserial-A50285BI 57600
