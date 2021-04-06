module top_module (
    input clk,
    input reset,
    input [7:0] d,
    output [7:0] q
);
    always @(negedge clk) q <= (reset) ? 8'h34 : d;

endmodule
