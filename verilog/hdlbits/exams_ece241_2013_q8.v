module top_module (
    input clk,
    input aresetn,    // Asynchronous active-low reset
    input x,
    output z ); 
    parameter WANT_ONE=0, WANT_ZERO=1, WANT_LAST=2;
    reg [1:0] state, next_state;
    always @(*)
        case(state)
            WANT_ONE: next_state = x ? WANT_ZERO : WANT_ONE;
            WANT_ZERO: next_state = x ? WANT_ZERO : WANT_LAST;
            WANT_LAST: next_state = x ? WANT_ZERO : WANT_ONE;
        endcase
    
    always @(posedge clk or negedge aresetn)
        if(~aresetn)
            state <= WANT_ONE;
        else
            state <= next_state;
    assign z = (state == WANT_LAST) & x;
    

endmodule
