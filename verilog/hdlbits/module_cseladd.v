module top_module(
    input [31:0] a,
    input [31:0] b,
    output [31:0] sum
);
    wire carry, ignore;
    wire [15:0] mux_hi, mux_lo;
    add16 low        (.a(a[15:0]),  .b(b[15:0]), .cin(1'b0),  .cout(carry),  .sum(sum[15:0]));
    add16 high_carry (.a(a[31:16]), .b(b[31:16]), .cin(1'b1), .cout(ignore), .sum(mux_hi));
    add16 low_carry  (.a(a[31:16]), .b(b[31:16]), .cin(1'b0), .cout(ignore), .sum(mux_lo));
    always @(*) begin
        if(carry) sum[31:16] = mux_hi;
        else      sum[31:16] = mux_lo;
    end

endmodule
