all: build flash

build:
	docker build . -f Dockerfile -t fpga_blink

flash:
	docker run --privileged fpga_blink
