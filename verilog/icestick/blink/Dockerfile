FROM vshyba/fusesoc:nextpnr-ice40

COPY . /app
WORKDIR /app
RUN fusesoc library add blink .
RUN fusesoc run --target=icestick fusesoc:learn:blink:0.1
RUN find .
ENTRYPOINT iceprog ./build/fusesoc_learn_blink_0.1/icestick-icestorm/fusesoc_learn_blink_0.1.bin
