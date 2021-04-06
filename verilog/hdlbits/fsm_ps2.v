module top_module(
    input clk,
    input [7:0] in,
    input reset,    // Synchronous reset
    output done); //
    parameter BYTE1=0, BYTE2=1, BYTE3=2, DONE=3;
    reg [1:0] state, next_state;
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
    end
 
    // Output logic
    assign done = state == DONE;

endmodule
