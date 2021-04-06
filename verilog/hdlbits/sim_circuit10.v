module top_module (
    input clk,
    input a,
    input b,
    output q,
    output state  );
    reg next_state;
    always @(*)
        case(state)
            0: next_state = (a & b) ? 1 : 0;
            1: next_state = (a | b) ? 1 : 0;
    	endcase
    always @(posedge clk)
        state <= next_state;
    always @(*)
        case(state)
            0: q = (~a & b) | (a & ~b);
            1: q = (~a & ~b) | (a & b);
        endcase

endmodule
