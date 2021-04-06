module top_module (
    input clk,
    input a,
    output q );
    parameter START=0, OFF=1;
    reg state, next_state;
    always @(*)
	    case(state)
		    START: next_state = a ? OFF : START;
		    OFF: next_state = a ? OFF : START;
	    endcase
    always @(posedge clk)
	    state <= next_state;
    assign q = ~(state == OFF);


endmodule
