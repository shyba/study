module top_module (
    input clk,
    input reset,   // Synchronous reset
    input s,
    input w,
    output z
);
    parameter A=0, B=1;
    reg state, next_state;
    reg [1:0] clocks, ones;
    always @(*)
        case(state)
            A: next_state = s ? B : A;
            B: next_state = B;
        endcase
    always @(posedge clk)
        case(state)
            A: begin
                clocks <= 0;
                ones <= 0;
            end
            B: begin
                clocks <= (clocks < 3) ? (clocks + 1) : 1;
                ones <= (clocks <3) ? (ones + w) : (w);
            end
        endcase

    
    always @(posedge clk)
        if(reset)
            state <= A;
        else
            state <= next_state;
   
    assign z = (state == B) & (ones == 2) & (clocks == 3);
        
    

endmodule
