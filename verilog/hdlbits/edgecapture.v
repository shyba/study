module top_module (
    input clk,
    input reset,
    input [31:0] in,
    output [31:0] out
);
    reg [31:0] last, state;
    always @(posedge clk) begin
        last <= in;
        state <= (reset) ? 32'd0 : state | (last & ~in);
    end
    assign out = state;

endmodule
