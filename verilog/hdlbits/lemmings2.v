module top_module(
    input clk,
    input areset,    // Freshly brainwashed Lemmings walk left.
    input bump_left,
    input bump_right,
    input ground,
    output walk_left,
    output walk_right,
    output aaah ); 
    parameter LEFT=0, RIGHT=1, FALLL=2, FALLR=3;
    reg [2:0] state, next_state;
    always @(*)
        case(state)
            LEFT:  next_state = ground ? (bump_left ? RIGHT : LEFT) : FALLL;
            RIGHT: next_state = ground ? (bump_right ? LEFT : RIGHT) : FALLR;
            FALLL: next_state = ground ? LEFT : FALLL;
            FALLR: next_state = ground ? RIGHT : FALLR;
        endcase
    
    always @(posedge clk or posedge areset) begin
        if(areset)
            state = LEFT;
        else
            state <= next_state;
    end
    
    assign aaah = state == FALLR | state == FALLL;
    assign walk_left = state == LEFT;
    assign walk_right = state == RIGHT;

endmodule
