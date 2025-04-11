.PHONY: prepare build run-example run-simple

prepare:
	rm -rf build
	mkdir build

build: prepare
	gcc -o build/assembler assembler/assembler.c

run-example: build
	./build/assembler examples/example.s build/example.bin
	cd vm && cargo run -- ../build/example.bin

run-simple: build
	./build/assembler examples/simple.s build/simple.bin
	cd vm && cargo run -- ../build/simple.bin
