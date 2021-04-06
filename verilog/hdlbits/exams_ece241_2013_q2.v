module top_module (
    input a,
    input b,
    input c,
    input d,
    output out_sop,
    output out_pos
); 
    assign out_sop = c & (d | (~a & ~b));
    assign out_pos = out_sop;

endmodule
