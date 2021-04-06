module top_module(
    input clk,
    input in,
    input reset,
    output out); //

    // State transition logic
    parameter A=0, B=1, C=2, D=3;
    reg [1:0] state, next;
    
    always @(*) begin
        case(state)
            A: next = in ? B : A;
            B: next = in ? B : C;
            C: next = in ? D : A;
            D: next = in ? B : C;
        endcase
    end

    // State flip-flops with synchronous reset
    always @(posedge clk)
        state <= (reset) ? A : next;
    
    assign out = state == D;

    // Output logic

endmodule
