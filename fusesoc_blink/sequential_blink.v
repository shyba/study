module top(input clk, output D1, output D2, output D3, output D4, output D5);

reg [23:0] divider;
reg [3:0] counter;
parameter clk_freq_hz = 12000000; //12mhz

always @(posedge clk) begin
        divider <= (divider < clk_freq_hz) ? (divider + 1) : 0;
        counter <= (divider == 0) ? (counter + 1) : counter;
end

initial begin
        divider = 1;
end

assign D1 = counter[0];
assign D2 = counter[1];
assign D3 = counter[2];
assign D4 = counter[3];
assign D5 = counter == 0;

endmodule
