module top_module (
    input too_cold,
    input too_hot,
    input mode,
    input fan_on,
    output heater,
    output aircon,
    output fan
); 
    assign heater = too_cold & mode;
    assign aircon = too_hot & ~mode;
    assign fan = fan_on | heater | aircon;

endmodule
