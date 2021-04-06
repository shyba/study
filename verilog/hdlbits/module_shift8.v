module top_module ( 
    input clk, 
    input [7:0] d, 
    input [1:0] sel, 
    output [7:0] q 
);
    wire [7:0] a;
    wire [7:0] b;
    wire [7:0] c;
    wire [7:0] select;
    my_dff8 (clk, d, a);
    my_dff8 (clk, a, b);
    my_dff8 (clk, b, c);
    
    always @(*) begin
        case(sel)
            2'b00: q = d;
            2'b01: q = a;
            2'b10: q = b;
            2'b11: q = c;
        endcase
    end
    

endmodule
