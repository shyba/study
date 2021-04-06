module top_module (
    input clk,
    input reset,      // Synchronous reset
    input data,
    output [3:0] count,
    output counting,
    output done,
    input ack );

    parameter S=0, S1=1, S11=2, S110=3, B0=4, B1=5, B2=6, B3=7, COUNT=8, WAIT=9;
    reg [3:0] state, next_state, counter;
    reg [9:0] thousand;
    wire done_counting, shift_ena;
    always @(*)
        case(state)
            S: next_state = data ? S1 : S;
            S1: next_state = data ? S11 : S;
            S11: next_state = data ? S11 : S110;
            S110: next_state = data ? B0 : S;
            B0: next_state = B1;
            B1: next_state = B2;
            B2: next_state = B3;
            B3: next_state = COUNT;
            COUNT: next_state = done_counting ? WAIT : COUNT;
            WAIT: next_state = ack ? S : WAIT;
        endcase
    always @(posedge clk)
        if (reset)
            state <= S;
        else begin
	    if (state == COUNT)
		    thousand <= (thousand == 0) ? 999 : (thousand - 1);
	    else
		    thousand <= 999;
            counter <= (shift_ena) ? {counter[2:0], data} : 
                       (state == COUNT & thousand == 0) ? (counter - 1 ) : counter;
            state <= next_state;
        end
    assign count = counter;
    assign counting = state == COUNT;
    assign shift_ena = (state == B0) | (state == B1) | (state == B2) | (state == B3);
    assign done_counting = (counter == 0) & (thousand == 0);
    assign done = state == WAIT;
endmodule
