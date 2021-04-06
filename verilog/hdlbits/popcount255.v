module top_module( 
    input [254:0] in,
    output [7:0] out );
    integer i;
    always @(*) begin
        out = 8'b00000000;
        for(i=0; i<255; i=i+1) begin
            if(in[i]) begin 
                out += 8'd1;
            end
        end
    end

endmodule
