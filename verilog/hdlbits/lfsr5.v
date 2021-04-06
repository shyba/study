module top_module(
    input clk,
    input reset,    // Active-high synchronous reset to 5'h1
    output [4:0] q
); 
    always @(posedge clk)
        q[0] <= (reset)? 1 : q[1];
    ff one   (clk, reset, 0^q[0], q[4]);
    ff two   (clk, reset, q[4], q[3]);
    ff three (clk, reset, q[3]^q[0], q[2]);
    ff four  (clk, reset, q[2], q[1]);

endmodule
module ff (input clk, input reset, input D, output reg Q);
    always @(posedge clk)
        Q <= (reset) ? 0 : D;
endmodule
        
