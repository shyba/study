module top_module (
    input clk,
    input reset,   // Synchronous active-high reset
    output [3:1] ena,
    output [15:0] q);
    reg [15:0] clock;
    assign ena[1] = clock[3:0] == 9;
    assign ena[2] = ena[1] && clock[7:4] == 9;
    assign ena[3] = ena[2] && clock[11:8] == 9;
    counter one   (clk,      1, reset, clock[3:0]);
    counter two   (clk, ena[1], reset, clock[7:4]);
    counter three (clk, ena[2], reset, clock[11:8]);
    counter four  (clk, ena[3], reset, clock[15:12]);
    assign q = clock;

endmodule
module counter (
    input clk,
    input enable,
    input reset,        // Synchronous active-high reset
    output [3:0] q);
    reg [3:0] counter;
    always @(posedge clk) counter <= (reset) ? 0 : 
        (!enable) ? counter :
        (counter < 9) ? (counter + 1) :
        0;

    assign q = counter;
    

endmodule
