NAME=example

compile:
	../build/assembler ../examples/${NAME}.s ../build/${NAME}.bin

flash:
	cargo r -- flash ../build/${NAME}.bin /dev/ttyUSB1

read:
	cargo r -- read /dev/ttyUSB1 ../build/${NAME}.bin

exec:
	cargo r -- exec /dev/ttyUSB1

exec-blocking:
	cargo r -- exec-blocking /dev/ttyUSB1

debug:
	cargo r -- debug /dev/ttyUSB1

run: compile flash exec-blocking read

run-dbg: compile flash exec debug read
