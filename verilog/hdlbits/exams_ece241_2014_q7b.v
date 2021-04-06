module top_module (
    input clk,
    input reset,
    output OneHertz,
    output [2:0] c_enable
);
    reg [3:0] clock1, clock2, clock3;
    assign c_enable[0] = 1;
    assign c_enable[1] = clock1 == 4'd9;
    assign c_enable[2] = c_enable[1] && clock2 == 4'd9;


    bcdcount counter0 (clk, reset, c_enable[0], clock1);
    bcdcount counter1 (clk, reset, c_enable[1], clock2);
    bcdcount counter2 (clk, reset, c_enable[2], clock3);

    
    assign OneHertz = c_enable[2] && clock3 == 9;

endmodule
