module top_module();
	reg clk, in;
	wire out;
	reg [2:0] s;
	initial begin
		clk=0; in=0;s=2;
		#10 s=6;
		#10 s=2;in=1;
		#10 s=7;in=0;
		#10 s=0;in=1;
		#30 in=0;
	end
	always #5 clk = ~clk;
	q7 under_test (clk, in, s, out);

endmodule
