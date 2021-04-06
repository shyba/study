module top_module (
    input clk,
    input reset,
    input [3:1] s,
    output fr3,
    output fr2,
    output fr1,
    output dfr
); 
    reg [2:0] state, next_state;
    reg increasing;
    always @(*)
        next_state = s;
    
    always @(posedge clk) begin
        if(reset) begin
            state <= 0;
        	increasing <= 0;
        end
        else begin
            state <= next_state;
            increasing <= (state < next_state) ? 1 : (state > next_state) ? 0 : increasing;
        end
    end
    assign fr3 = ~state[0];
    assign fr2 = ~state[0] | ~state[1];
    assign fr1 = ~state[0] | ~state[1] | ~state[2];
    assign dfr = ~state[0] | ~increasing;

endmodule
