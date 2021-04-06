module top_module ( input clk, input d, output q );
    wire a, b;
    my_dff(clk, d, a);
    my_dff(clk, a, b);
    my_dff(clk, b, q);
endmodule
