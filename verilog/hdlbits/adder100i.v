module adder_module( 
    input a, b, cin,
    output cout, sum );
    assign cout = (a & b) | ((a | b) & cin);
    assign sum = a^b^cin;
endmodule

module top_module( 
    input [99:0] a, b,
    input cin,
    output [99:0] cout,
    output [99:0] sum );
    generate
        adder_module addm (a[0], b[0], cin, cout[0], sum[0]);
        genvar i;
        for(i=1; i<100; i=i+1) begin : generate_adders
            adder_module addm (a[i], b[i], cout[i-1], cout[i], sum[i]);
        end
    endgenerate
            

endmodule
