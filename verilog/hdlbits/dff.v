module top_module (
    input clk,    // Clocks are used in sequential circuits
    input d,
    output reg q );//
    always @(posedge clk) q <= d;

    // Use a clocked always block
    //   copy d to q at every positive edge of clk
    //   Clocked always blocks should use non-blocking assignments

endmodule
