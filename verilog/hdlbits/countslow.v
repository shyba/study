module top_module (
    input clk,
    input slowena,
    input reset,
    output [3:0] q);
    reg [3:0] counter;
    assign q = counter;
    always @(posedge clk) counter <= (reset) ? 0 :
        (slowena & counter < 9) ? (counter + 1) :
        (slowena & counter > 8) ? 0 : counter;

endmodule
