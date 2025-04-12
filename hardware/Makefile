.PHONY: build upload docker

SOURCES=comm.v uart.v mux.v main.v

build: $(SOURCES)
	mkdir -p build
	rm -rf build
	mkdir build
	yosys -p "read_verilog $(SOURCES) ; synth_gowin -json build/main.json"
	nextpnr-himbaechel --json build/main.json --write build/main_pnr.json --device GW1NR-LV9QN88PC6/I5 --vopt family=GW1N-9C --vopt cst=tangnano9k.cst
	gowin_pack -d GW1N-9C -o build/pack.fs build/main_pnr.json

upload:
	openFPGALoader -b tangnano9k build/pack.fs

upload-nv:
	openFPGALoader -b tangnano9k -f build/pack.fs

run:
	docker run -v .:/mnt/app -w /mnt/app -it cbergschneider/gowinc make build
	make upload

run-nv:
	docker run -v .:/mnt/app -w /mnt/app -it cbergschneider/gowinc make build
	make upload-nv

test_uart:
	mkdir -p test
	iverilog -o test/uart_test.o -s uart_tb uart.v uart_tb.v
	cd test/ && vvp uart_test.o

test_mux:
	mkdir -p test
	iverilog -o test/mux_test.o -s mux_tb mux.v mux_tb.v
	cd test/ && vvp mux_test.o
