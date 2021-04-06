module top_module (
    input clk,
    input [2:0] y,
    input x,
    output Y0,
    output z
);
    reg [2:0] next_state;
    always @(*)
        case(y)
            3'b000: next_state = ~x ? 3'b000 : 3'b001;
            3'b001: next_state = ~x ? 3'b001 : 3'b100;
            3'b010: next_state = ~x ? 3'b010 : 3'b001;
            3'b011: next_state = ~x ? 3'b001 : 3'b010;
            3'b100: next_state = ~x ? 3'b011 : 3'b100;
        endcase

    assign z = (y == 3'b011) | (y == 3'b100);
    assign Y0 = next_state[0];

endmodule
