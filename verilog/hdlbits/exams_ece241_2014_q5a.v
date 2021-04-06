module top_module (
    input clk,
    input areset,
    input x,
    output z
); 
    parameter WANT_ONE=0, REVERSE=1;
    reg state, next_state;
    
    always @(*)
        case(state)
            WANT_ONE: next_state = x ? REVERSE : WANT_ONE;
            REVERSE: next_state = REVERSE;
        endcase
    
    always @(posedge clk, posedge areset) begin
        if(areset) begin
            state <= WANT_ONE;
            z <= 0;
        end
        else begin
            state <= next_state;
            z <= (state == REVERSE) ? ~x : x;
        end
    end

endmodule
