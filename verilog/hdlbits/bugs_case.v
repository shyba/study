module top_module (
    input [7:0] code,
    output reg [3:0] out,
    output reg valid=1 );//

     always @(*)
        case (code)
            8'h45: out = 0;
            8'h16: out = 1;
            8'h1e: out = 2;
            8'h26: out = 3;
            8'h25: out = 4;
            8'h2e: out = 5;
            8'h36: out = 6;
            8'h3d: out = 7;
            8'h3e: out = 8;
            8'h46: out = 9;
            default: out = 0;
        endcase
    assign valid = ~(out == 0 & code != 8'h45);

endmodule
