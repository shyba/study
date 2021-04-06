module top_module (
    input clk,
    input d,
    output q
);
    reg pos, neg;
    always @(posedge clk) pos <= neg^d;
    always @(negedge clk) neg <= pos^d;
    assign q = pos ^ neg;
      


endmodule
