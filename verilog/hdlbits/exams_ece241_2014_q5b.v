module top_module (
    input clk,
    input areset,
    input x,
    output z
); 
    parameter A=2'b01, B=2'b10;
    reg [1:0] state, next_state;
    always @(*)
        case(state)
            A: next_state = x ? B : A;
            B: next_state = B;
        endcase
    
    always @(posedge clk or posedge areset)
        if(areset)
            state <= A;
        else
            state <= next_state;
    
    always @(*)
        case(state)
            A: z = x;
            B: z = ~x;
        endcase

endmodule
