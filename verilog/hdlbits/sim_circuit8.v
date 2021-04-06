module top_module (
    input clock,
    input a,
    output p,
    output q );
    always @(*)
        case(clock)
            0: q = p;
            1: p = a;
        endcase
endmodule
