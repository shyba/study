module top_module (
    input clk,
    input reset,      // Synchronous active-high reset
    output [3:0] q);
    reg [3:0] counter;
    always @(posedge clk) counter = (reset) ? 0 : counter + 1;
    assign q = counter;

endmodule
