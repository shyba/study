module top_module (
    input [5:0] y,
    input w,
    output Y1,
    output Y3
);
    assign Y1 = y[0] & w;
    assign Y3 = ~w & (y[1] | y[2] | y[4] | y[5]);

endmodule
