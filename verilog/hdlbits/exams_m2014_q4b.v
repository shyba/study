module top_module (
    input clk,
    input d, 
    input ar,   // asynchronous reset
    output q);
    always @(posedge clk or posedge ar) q = (ar) ? 0 : d;

endmodule
