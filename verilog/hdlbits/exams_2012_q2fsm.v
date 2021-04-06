module top_module (
    input clk,
    input reset,   // Synchronous active-high reset
    input w,
    output z
);
    parameter A=0, B=1, C=2, D=3, E=4, F=5;
    reg [2:0] state, next_state;
    always @(*)
        case(state)
            A: next_state = w ? B : A;
            B: next_state = w ? C : D;
            C: next_state = w ? E : D;
            D: next_state = w ? F : A;
            E: next_state = w ? E : D;
            F: next_state = w ? C : D;
        endcase
    always @(posedge clk)
        if(reset) state <= A;
        else state <= next_state;
    
    assign z = (state == E) | (state == F);

endmodule

