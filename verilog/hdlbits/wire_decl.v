`default_nettype none
module top_module(
    input a,
    input b,
    input c,
    input d,
    output out,
    output out_n   ); 
	wire a_and_b, c_and_d, after_or;
    assign a_and_b = a && b;
    assign c_and_d = c && d;
    assign after_or = a_and_b || c_and_d;
    assign out = after_or;
	assign out_n = ~after_or;
endmodule
