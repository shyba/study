module A (input x, input y, output z);
    assign z = (x^y) & x;
endmodule

module B ( input x, input y, output z );
    assign z = (~x & ~y) | (x & y);
endmodule

module top_module (input x, input y, output z);
    wire [1:0] ab_and, ab_or;
    A IA1(x, y, ab_or[0]);
    B IB1(x, y, ab_or[1]);
    
    A IA2(x, y, ab_and[0]);
    B IB2(x, y, ab_and[1]);
    assign z = (ab_and[0] & ab_and[1]) ^ (ab_or[0] | ab_or[1]);

endmodule
