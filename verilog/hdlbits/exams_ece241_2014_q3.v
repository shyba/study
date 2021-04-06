module top_module (
    input c,
    input d,
    output [3:0] mux_in
); 
    assign mux_in = (~c & ~d) ? 4'b0100 :
        (~c & d) ? 4'b0001 :
        (c & d) ?  4'b1001 :
        4'b0101;

endmodule
