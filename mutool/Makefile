flash:
	cargo r -- flash ../build/example.bin /dev/ttyUSB1

read:
	cargo r -- read /dev/ttyUSB1 ../build/example.bin

exec:
	cargo r -- exec /dev/ttyUSB1

debug:
	cargo r -- debug /dev/ttyUSB1

all: flash read exec read
all-dbg: flash exec debug
