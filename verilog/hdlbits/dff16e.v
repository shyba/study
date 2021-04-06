module top_module (
    input clk,
    input resetn,
    input [1:0] byteena,
    input [15:0] d,
    output [15:0] q
);
    always @(posedge clk) begin
        q[7:0]  <= (~resetn) ? 0 : (byteena[0]) ? d[7:0] : q[7:0];
        q[15:8] <= (~resetn) ? 0 : (byteena[1]) ? d[15:8] : q[15:8];
    end

endmodule
