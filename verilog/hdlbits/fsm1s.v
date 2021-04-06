// Note the Verilog-1995 module declaration syntax here:
module top_module(clk, reset, in, out);
    input clk;
    input reset;    // Synchronous reset to state B
    input in;
    output out;//  
    reg out;

    // Fill in state name declarations
    localparam B=1,A=0;

    reg present_state, next_state;
    
    always @(posedge clk) begin
        case (present_state)
        	A: next_state = (in) ? A : B;
            B: next_state = (in) ? B : A;
            default: next_state = A;
        endcase
        if (reset) begin  
            present_state <= B;
        end else begin


            // State flip-flops
            present_state <= next_state;   
        end
    end
    assign out = present_state == B;

endmodule
