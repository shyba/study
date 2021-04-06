module top_module(
    input clk,
    input reset,    // Active-high synchronous reset to 32'h1
    output [31:0] q
); 
    always @(posedge clk) begin
        q[0] <= (reset) ? 1 : q[1]^q[0];
        q[1] <= (reset) ? 0 : q[2]^q[0];
        q[2] <= (reset) ? 0 : q[3];
        q[3] <= (reset) ? 0 : q[4];
        q[4] <= (reset) ? 0 : q[5];
        q[5] <= (reset) ? 0 : q[6];
        q[6] <= (reset) ? 0 : q[7];
        q[7] <= (reset) ? 0 : q[8];
        q[8] <= (reset) ? 0 : q[9];
        q[9] <= (reset) ? 0 : q[10];
        q[10] <= (reset) ? 0 : q[11];
        q[11] <= (reset) ? 0 : q[12];
        q[12] <= (reset) ? 0 : q[13];
        q[13] <= (reset) ? 0 : q[14];
        q[14] <= (reset) ? 0 : q[15];
        q[15] <= (reset) ? 0 : q[16];
        q[16] <= (reset) ? 0 : q[17];
        q[17] <= (reset) ? 0 : q[18];
        q[18] <= (reset) ? 0 : q[19];
        q[19] <= (reset) ? 0 : q[20];
        q[20] <= (reset) ? 0 : q[21];
        q[21] <= (reset) ? 0 : q[22]^q[0];
        q[22] <= (reset) ? 0 : q[23];
        q[23] <= (reset) ? 0 : q[24];
        q[24] <= (reset) ? 0 : q[25];
        q[25] <= (reset) ? 0 : q[26];
        q[26] <= (reset) ? 0 : q[27];
        q[27] <= (reset) ? 0 : q[28];
        q[28] <= (reset) ? 0 : q[29];
        q[29] <= (reset) ? 0 : q[30];
        q[30] <= (reset) ? 0 : q[31];
        q[31] <= (reset) ? 0 : q[0];
    end

endmodule
