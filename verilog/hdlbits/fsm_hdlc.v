module top_module(
    input clk,
    input reset,    // Synchronous reset
    input in,
    output disc,
    output flag,
    output err);
    parameter NONE=0, ONE=1, TWO=2, THREE=3, FOUR=4, FIVE=5, SIX=6, ERROR=7, DISCARD=8, FLAG=9;
    reg [3:0] state, next_state;
    always @(*)
        case(state)
            NONE: next_state = in ? ONE : NONE;
            ONE: next_state = in ? TWO : NONE;
            TWO: next_state = in ? THREE : NONE;
            THREE: next_state = in ? FOUR : NONE;
            FOUR: next_state = in ? FIVE : NONE;
            FIVE: next_state = in ? SIX : DISCARD;
            SIX: next_state = in ? ERROR : FLAG;
            ERROR: next_state = in ? ERROR : NONE;
            DISCARD: next_state = in ? ONE : NONE;
            FLAG: next_state = in ? ONE : NONE;
        endcase
    
    always @(posedge clk)
        if(reset)
            state <= NONE;
    	else
            state <= next_state;
    assign disc = state == DISCARD;
    assign flag = state == FLAG;
    assign err = state == ERROR;

endmodule
