module top_module ();
    reg clk, reset, t;
    wire q;
    initial begin
            clk=0; reset=0; t=0;
            #5 reset=1;
            #5 reset=0;
            #5 t=1;
            #5 reset=1;
    end
    always #5 clk = ~clk;
    tff under_test (clk, reset, t, q);

endmodule
