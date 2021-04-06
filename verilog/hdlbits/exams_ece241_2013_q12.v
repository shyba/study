module top_module (
    input clk,
    input enable,
    input S,
    input A, B, C,
    output Z ); 
    reg [7:0] Q;
    assign Z = Q[(A * 4) + (B * 2) + C];
    always @(posedge clk) begin
        Q <= (enable) ? {Q[6:0], S} : Q;
    end

endmodule
