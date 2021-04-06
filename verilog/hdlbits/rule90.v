module top_module(
    input clk,
    input load,
    input [511:0] data,
    output [511:0] q ); 
    reg [511:0] state;
    always @(posedge clk) begin
        if(load)
            state <= data;
        else
            state <= (state << 1) ^ (state >> 1);
    end
    assign q = state;

endmodule
