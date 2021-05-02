flash: sequential_blink.bin
	iceprog sequential_blink.bin

sequential_blink.blif: sequential_blink.v
	yosys -p "synth_ice40 -blif sequential_blink.blif" sequential_blink.v

sequential_blink.txt: sequential_blink.blif
	arachne-pnr -d 1k -p boards/icestick.pcf sequential_blink.blif -o sequential_blink.txt

sequential_blink.bin: sequential_blink.txt
	icepack sequential_blink.txt sequential_blink.bin
