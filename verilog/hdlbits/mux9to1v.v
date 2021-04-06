module top_module( 
    input [15:0] a, b, c, d, e, f, g, h, i,
    input [3:0] sel,
    output [15:0] out );
    assign out = (sel == 0) ? a :
        (sel == 1) ? b :
        (sel == 2) ? c :
        (sel == 3) ? d :
        (sel == 4) ? e :
        (sel == 5) ? f :
        (sel == 6) ? g :
        (sel == 7) ? h :
        (sel == 8) ? i :
        16'hffff;

endmodule
