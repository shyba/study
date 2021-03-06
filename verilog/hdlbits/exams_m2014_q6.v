module top_module (
    input clk,
    input reset,     // synchronous reset
    input w,
    output z);
    parameter A=0, B=1, C=2, D=3, E=4, F=5;
    reg [2:0] state, next_state;
    always @(*)
        case(state)
            A: next_state = w ? A : B;
            B: next_state = w ? D : C;
            C: next_state = w ? D : E;
            D: next_state = w ? A : F;
            E: next_state = w ? D : E;
            F: next_state = w ? D : C;
        endcase
    always @(posedge clk)
        if(reset) state <= A;
        else state <= next_state;
    
    assign z = (state == E) | (state == F);

endmodule
