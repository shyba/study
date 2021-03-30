module top_module(
    input clk,
    input load,
    input [255:0] data,
    output [255:0] q ); 
    reg state[15:0][15:0];
    integer x,y; 
    genvar j, k;
    generate
        for(j=0;j<16;j++) begin:verilog
            for(k=0;k<16;k++) begin:sucks
                assign q[(k*16) + j] = state[k][j];
            end
        end
    endgenerate


    always @(posedge clk) begin
        if(load) begin
            for(y=0;y<16;y++)
                for(x=0;x<16;x++)
                    state[y][x] <= data[(y*16) + x];
        end
        else begin
            integer pos, yup, ydown, xright, xleft, sum;
            for(y=0;y<16;y++) begin//row
                for(x=0;x<16;x++) begin//column
                    ydown = (y+1)%16;
                    yup = (y>0)?(y-1):15;
                    xright = (x+1)%16;
                    xleft = (x>0)?(x-1):15;
                    sum = state[yup][x]     + state[ydown][x]    + state[ydown][xright] + state[ydown][xleft] +
                          state[yup][xleft] + state[yup][xright] + state[y][xright]     + state[y][xleft];
                    state[y][x] <= (sum == 2) ? state[y][x] : (sum == 3) ? 1 : 0;
                end
            end
        end
    end
endmodule
