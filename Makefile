.PHONY: prepare build run-example run-simple

prepare:
	rm -rf build
	mkdir build

build: prepare
	gcc -o build/assembler assembler/assembler.c
	cd vm && cargo build
	cp vm/target/debug/vm build/vm

run-example: build
	./build/assembler examples/example.s build/example.bin
	./build/vm build/example.bin

run-simple: build
	./build/assembler examples/simple.s build/simple.bin
	./build/vm ../build/simple.bin
