module top_module(
    input clk,
    input [7:0] in,
    input reset,    // Synchronous reset
    output [23:0] out_bytes,
    output done); //

    // FSM from fsm_ps2
    parameter BYTE1=0, BYTE2=1, BYTE3=2, DONE=3;
    reg [1:0] state, next_state;
    reg [23:0] accumulate;
    // State transition logic (combinational)
    always @(*)
        case(state)
            BYTE1: next_state = in[3] ? BYTE2 : BYTE1;
            BYTE2: next_state = BYTE3;
            BYTE3: next_state = DONE;
            DONE:  next_state = in[3] ? BYTE2 : BYTE1;
        endcase
    
    // State flip-flops (sequential)
    always @(posedge clk) begin
        if(reset)
            state <= BYTE1;
        else
            state <= next_state;
        accumulate <= {accumulate[15:0], in};
    end
 
    // Output logic
    assign done = state == DONE;
    assign out_bytes = (done) ? accumulate : 0;

endmodule
