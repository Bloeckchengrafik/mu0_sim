.PHONY: build upload docker

build:
	mkdir -p build
	rm -rf build
	mkdir build
	yosys -p "read_verilog uart.v ; synth_gowin -json build/uart.json"
	nextpnr-himbaechel --json build/uart.json --write build/pnruart.json --device GW1NR-LV9QN88PC6/I5 --vopt family=GW1N-9C --vopt cst=tangnano9k.cst
	gowin_pack -d GW1N-9C -o build/pack.fs build/pnruart.json

upload:
	sudo openFPGALoader -b tangnano9k build/pack.fs

upload-nv:
	sudo openFPGALoader -b tangnano9k -f build/pack.fs

run:
	docker run -v .:/mnt/app -w /mnt/app -it cbergschneider/gowinc make build
	make upload

run-nv:
	docker run -v .:/mnt/app -w /mnt/app -it cbergschneider/gowinc make build
	make upload-nv

test_uart:
	mkdir -p test
	iverilog -o test/uart_test.o -s test uart.v uart_tb.v
	cd test/ && vvp uart_test.o
