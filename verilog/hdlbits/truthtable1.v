module top_module( 
    input x3,
    input x2,
    input x1,  // three inputs
    output f   // one output
);
    assign f = (~x1 & x2 & ~x3) | (x1 & x2 & ~x3) | (x1 & ~x2 & x3) | (x1 & x2 & x3);

endmodule
