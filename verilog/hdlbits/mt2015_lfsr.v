module top_module (
	input [2:0] SW,      // R
	input [1:0] KEY,     // L and clk
	output [2:0] LEDR);  // Q
    always @(posedge KEY) begin
        LEDR[0] <= (KEY) ? SW[0] : LEDR[2];
        LEDR[1] <= (KEY) ? SW[1] : LEDR[0];
        LEDR[2] <= (KEY) ? SW[2] : LEDR[1] ^ LEDR[2];
    end


endmodule