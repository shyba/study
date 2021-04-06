module top_module (
    input clk,
    input reset,
    output [3:0] q);
    reg [3:0] counter = 1;
    assign q = counter;
    always @(posedge clk) counter <= (!reset && counter < 10) ? (counter + 1) : 1;

endmodule
