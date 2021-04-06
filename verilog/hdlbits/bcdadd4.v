module top_module( 
    input [15:0] a, b,
    input cin,
    output cout,
    output [15:0] sum );
    wire [2:0] carry;
    bcd_fadd many[3:0] (a, b, {carry, cin}, {cout, carry}, sum);

endmodule
