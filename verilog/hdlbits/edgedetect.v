module top_module (
    input clk,
    input [7:0] in,
    output [7:0] pedge
);
    reg [7:0] state;
    always @(posedge clk) begin
        state <= in;
        pedge <= ~state & in;
    end
        
        

endmodule
