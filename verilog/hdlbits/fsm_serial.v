module top_module(
    input clk,
    input in,
    input reset,    // Synchronous reset
    output done
); 

    //state is 0-7
    reg [3:0] state, next_state;
    // State transition logic (combinational)
    always @(*)
        case(state)
            0: next_state = ~in ? 1 : 0;
            9: next_state = in ? 0 : 10;
            10: next_state = in ? 0 : 10;
            default: next_state = state + 1;
        endcase
    
    // State flip-flops (sequential)
    always @(posedge clk) begin
        if(reset)
            state <= 0;
        else
            state <= next_state;
        done <= (state == 9) & (next_state == 0);
    end

endmodule
