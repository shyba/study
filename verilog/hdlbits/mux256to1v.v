module top_module( 
    input [1023:0] in,
    input [7:0] sel,
    output [3:0] out );
    assign out = in[((1+sel)*4)-1 -:4];

endmodule
