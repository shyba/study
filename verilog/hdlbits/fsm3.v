module top_module(
    input clk,
    input in,
    input areset,
    output out); //

    // State transition logic
    localparam A=0, B=1, C=2, D=3;
    reg [3:0] state;
    always @(posedge clk or posedge areset) begin
        if(areset)
            state = 4'b0001;
        else begin
            state[A] <= ~in & (state[C] | state[A]);
            state[B] <= in & ~state[C];
            state[C] <= ~in & (state[B] | state[D]);
            state[D] <= (state[C] & in);
        end
    end
    assign out = state[D];
        

    // State flip-flops with asynchronous reset

    // Output logic

endmodule
