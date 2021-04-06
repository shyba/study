module top_module(
    input clk,
    input areset,    // Freshly brainwashed Lemmings walk left.
    input bump_left,
    input bump_right,
    input ground,
    input dig,
    output walk_left,
    output walk_right,
    output aaah,
    output digging ); 

    parameter LEFT=0, RIGHT=1, DIG_L=2, DIG_R=3, FALL_L=4, FALL_R=5, SPLAT=6;
    reg [2:0] state, next_state;
    reg [4:0] falling_counter;
    
    always @(*)
        case(state)
            LEFT:  next_state = ground ? (dig ? DIG_L : (bump_left ? RIGHT : LEFT)) : FALL_L;
            RIGHT: next_state = ground ? (dig ? DIG_R : (bump_right ? LEFT : RIGHT)) : FALL_R;
            DIG_L: next_state = ground ? DIG_L : FALL_L;
            DIG_R: next_state = ground ? DIG_R : FALL_R;
            FALL_L: next_state = (ground) ? ((falling_counter < 20) ? LEFT : SPLAT) : FALL_L;
            FALL_R: next_state = (ground) ? ((falling_counter < 20) ? RIGHT : SPLAT) : FALL_R;
            SPLAT: next_state = SPLAT;
        endcase
    
    always @(posedge clk or posedge areset) begin
        if(areset)
            state <= LEFT;
        else begin
            state <= next_state;
            if(state==FALL_L | state==FALL_R)
                falling_counter = (falling_counter < 21) ? (falling_counter + 1) : falling_counter;
            else
                falling_counter = 0;
        end
     
    end
    assign aaah = state == FALL_L | state == FALL_R;
    assign digging = state == DIG_L | state == DIG_R;
    assign walk_left = state == LEFT;
    assign walk_right = state == RIGHT;
endmodule
