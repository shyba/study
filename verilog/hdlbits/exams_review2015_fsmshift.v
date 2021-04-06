module top_module (
    input clk,
    input reset,      // Synchronous reset
    output shift_ena);
    reg [2:0] counter;
    always @(posedge clk)
        if(reset)
            counter <= 0;
        else
            counter <= counter[2] ? counter : (counter + 1);
    assign shift_ena = counter != 4;

endmodule
