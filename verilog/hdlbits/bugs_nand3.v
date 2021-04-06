module top_module (input a, input b, input c, output out);
    wire ignored=1, inverted;
    andgate inst1 ( inverted, a, b, c, ignored, ignored );
    assign out = ~inverted;

endmodule
