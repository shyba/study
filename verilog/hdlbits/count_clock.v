module top_module(
    input clk,
    input reset,
    input ena,
    output pm,
    output [7:0] hh,
    output [7:0] mm,
    output [7:0] ss); 
    reg [3:0] r1h, r0h, r1m, r0m, r1s, r0s; // register <digit> <field>
    reg rpm;
    wire rollover, hour_reset, toggle_pm, ignored;
    wire [3:0] hour_reset_value_hi, hour_reset_value_lo;
    assign hh = {r1h, r0h};
    assign mm = {r1m, r0m};
    assign ss = {r1s, r0s};
    assign hour_reset = reset || (enable[3] && r0h == 2 && r1h == 1);
    assign hour_reset_value_hi = (reset) ? 1 : 0;
    assign hour_reset_value_lo = (reset) ? 2 : 1;
    
    wire [4:0] enable;
    wire [3:0] normal_reset;
    counter sec_lo (clk, reset, 0, ena,       9, enable[0], r0s);
    counter sec_hi (clk, reset, 0, enable[0], 5, enable[1], r1s);
    counter min_lo (clk, reset, 0, enable[1], 9, enable[2], r0m); 
    counter min_hi (clk, reset, 0, enable[2], 5, enable[3], r1m);
    counter hhr_lo (clk, hour_reset, hour_reset_value_lo, enable[3], 9, enable[4], r0h);
    counter hhr_hi (clk, hour_reset, hour_reset_value_hi, enable[4], 1, ignored,       r1h);
	
    always @(posedge clk) begin
        rpm <= (reset) ? 0 : (enable[3] && r0h == 1 && r1h == 1) ? ~rpm : rpm;
    end
    assign pm = rpm;

        

endmodule
module counter(input clk, input reset, input [3:0] reset_to, input ena, 
               input [3:0] max, output overflow, output reg [3:0] counter);
    assign overflow = ena && (counter == max);
    always @(posedge clk) begin
        if(reset) begin
            counter <= reset_to;
        end
        else if (ena) begin
            if (counter < max) begin
                counter <= counter + 1;
            end
            else begin
                counter <= 0;
            end
        end
    end
endmodule
