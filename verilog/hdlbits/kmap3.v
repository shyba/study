module top_module(
    input a,
    input b,
    input c,
    input d,
    output out  ); 
    assign out = (~a & b) ? 0 :
        (~c & d) ? 0 :
        (a | b | c | d);

endmodule
