module top_module(
    input clk,
    input in,
    input reset,    // Synchronous reset
    output [7:0] out_byte,
    output done
); //
    //state is 0-11
    reg [3:0] state, next_state;
    reg odd;
    
    // State transition logic (combinational)
    always @(*)
        case(state)
            0: next_state = ~in ? 1 : 0;
            10: next_state = in ? 0 : 11; // stop + 1 = 0, stop + 0 = error
            11: next_state = in ? 0 : 11; // error
            default: next_state = state + 1;
        endcase
    
    parity(clk, (state==0), in, odd);
    
    // State flip-flops (sequential)
    always @(posedge clk) begin
        if(reset)
            state <= 0;
        else
            state <= next_state;
        done <= (state == 10) & (next_state == 0) & (in == odd);
        out_byte <= (state > 0 & state < 9) ? {in, out_byte[7:1]} : out_byte;
    end

endmodule

