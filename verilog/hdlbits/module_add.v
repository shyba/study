module top_module(
    input [31:0] a,
    input [31:0] b,
    output [31:0] sum
);
    wire carry, ignored;
    add16 lower (.cin(1'b0), .cout(carry), .a(a[15:0]), .b(b[15:0]), .sum(sum[15:0]));
    add16 upper (.cin(carry), .cout(ignored), .a(a[31:16]), .b(b[31:16]), .sum(sum[31:16]));

endmodule
