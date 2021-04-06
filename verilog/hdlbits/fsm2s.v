module top_module(
    input clk,
    input reset,    // Synchronous reset to OFF
    input j,
    input k,
    output out); //  

    parameter OFF=0, ON=1; 
    reg state, next_state;

    always @(*) begin
        case(state)
            ON: next_state = k ? OFF : ON;
            OFF: next_state = j ? ON : OFF;
        endcase
    end

    always @(posedge clk) begin
        if(reset)
            state <= OFF;
        else
            state <= next_state;
        // State flip-flops with synchronous reset
    end

    // Output logic
            assign out = (state == ON);

endmodule
