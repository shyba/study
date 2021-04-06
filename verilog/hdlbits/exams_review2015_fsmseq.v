module top_module (
    input clk,
    input reset,      // Synchronous reset
    input data,
    output start_shifting);
    parameter START=0, WANT_ONE=1, WANT_ZERO=2, WANT_LAST_ONE=3, FINISHED=4;
    reg [3:0] state, next_state;
    always @(*)
        case(state)
            START: next_state = data ? WANT_ONE : START;
            WANT_ONE: next_state = data ? WANT_ZERO : START;
            WANT_ZERO: next_state = ~data ? WANT_LAST_ONE : WANT_ZERO;
            WANT_LAST_ONE: next_state = data ? FINISHED : START;
            FINISHED: next_state = FINISHED;
        endcase
    
    always @(posedge clk)
        if(reset)
            state <= START;
        else
            state <= next_state;
    assign start_shifting = state == FINISHED;

endmodule
