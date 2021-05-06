FROM vshyba/fusesoc:nextpnr-ice40

COPY . /app
WORKDIR /app
RUN fusesoc library add blink .
RUN fusesoc run --target=icestick fusesoc:learn:blink:0.1
