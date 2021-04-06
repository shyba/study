module top_module (
    input clk,
    input shift_ena,
    input count_ena,
    input data,
    output [3:0] q);
    reg [3:0] state;
    always @(posedge clk) begin
        if(shift_ena)
            state <= {state[2:0], data};
        else if (count_ena)
            state <= state - 1;
    end
    assign q = state;
            

endmodule
