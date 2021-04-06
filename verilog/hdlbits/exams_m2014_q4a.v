module top_module (
    input d, 
    input ena,
    output q);
    always @(ena) q <= (ena) ? d : q;

endmodule
