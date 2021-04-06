module top_module (
    input [3:0] x,
    input [3:0] y, 
    output [4:0] sum);
    wire [2:0] carry;
    fadd many1 (x[0], y[0], 0,        carry[0], sum[0]);
    fadd many2 (x[1], y[1], carry[0], carry[1], sum[1]);
    fadd many3 (x[2], y[2], carry[1], carry[2], sum[2]);
    fadd many4 (x[3], y[3], carry[2], sum[4],   sum[3]);


endmodule
module fadd( 
    input a, b, cin,
    output cout, sum );
    assign cout = (a & b) | ((a | b) & cin);
    assign sum = a^b^cin;
endmodule