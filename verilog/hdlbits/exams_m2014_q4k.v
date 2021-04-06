module top_module (
    input clk,
    input resetn,   // synchronous reset
    input in,
    output out);
    reg [3:0] ff;
    always @(posedge clk) begin
        ff <= (~resetn) ? 0 : {in, ff[2:0]};
        out <= (~resetn) ? 0 : ff[3];
    end
endmodule
