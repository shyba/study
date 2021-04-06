module top_module(
    input [31:0] a,
    input [31:0] b,
    input sub,
    output [31:0] sum
);
    wire carry, ignore;
    wire [31:0] high_b;
    assign high_b = b ^ {32{sub}};
    add16 low  (.a(a[15:0]),  .b(high_b[15:0]),  .cin(sub),   .cout(carry),  .sum(sum[15:0]));
    add16 high (.a(a[31:16]), .b(high_b[31:16]), .cin(carry), .cout(ignore), .sum(sum[31:16]));

endmodule
