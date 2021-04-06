module top_module (
    input clk,
    input resetn,    // active-low synchronous reset
    input x,
    input y,
    output f,
    output g
); 
    parameter START=0, SET_F=1, MONITOR_X=2, ENDING=3, END_ONE=4, WAIT_ZERO=5, END_ZERO=6;
    reg [3:0] state, next_state;
    reg [2:0] shift_x;
    
    always @(*)
        case(state)
            START: next_state = SET_F;
            SET_F: next_state = MONITOR_X;
            MONITOR_X: next_state = (shift_x == 3'b101) ? ENDING : MONITOR_X;
            ENDING: next_state = y ? END_ONE : WAIT_ZERO;
            WAIT_ZERO: next_state = y ? END_ONE : END_ZERO;
            END_ZERO: next_state = END_ZERO;
            END_ONE: next_state = END_ONE;
        endcase
    always @(posedge clk) begin
        if(~resetn) state <= START;
        else begin
            state <= next_state;
            shift_x = (state == MONITOR_X) ? {shift_x[1:0], x} : 0;
        end
    end
    
    assign f = (state == SET_F);
    assign g = (next_state == ENDING) | (state == ENDING) | (state == END_ONE);

endmodule
