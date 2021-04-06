module top_module (
    input [3:1] y,
    input w,
    output Y2);
    always @(*)
        case(y)
            0: Y2 = 0;
            1: Y2 = 1;
            2: Y2 = w;
            3: Y2 = 0;
            4: Y2 = w;
            5: Y2 = 1;
        endcase

endmodule
