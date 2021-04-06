module top_module (
    input clk,
    input reset,
    input enable,
    output [3:0] Q,
    output c_enable,
    output c_load,
    output [3:0] c_d
); //
    reg [3:0] prev;
    assign c_enable = enable;
    assign c_load = reset || (enable && prev > 11);
    assign c_d = (reset) ? 1 : (c_enable && prev > 11) ? 1 : 0;
    count4 the_counter (clk, c_enable, c_load, c_d, prev);
	assign Q = prev;


endmodule
