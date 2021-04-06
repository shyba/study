module fadd( 
    input a, b, cin,
    output cout, sum );
    assign cout = (a & b) | ((a | b) & cin);
    assign sum = a^b^cin;
endmodule
module top_module( 
    input [2:0] a, b,
    input cin,
    output [2:0] cout,
    output [2:0] sum );
    wire [3:0] carry;
    assign carry[0] = cin;
    fadd many[2:0] (a, b, carry[2:0], carry[3:1], sum);
    assign cout = carry[3:1];

endmodule
