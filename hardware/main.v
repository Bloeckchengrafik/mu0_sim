module top (
    input clk,
    input btn1,
    input btn2,
    output [5:0] led
);

    localparam WAIT_TIME = 13500000;
    reg [5:0] ledCounter = 0;
    reg [23:0] clockCounter = 0;
    wire paused;
    wire reset;
    togglebutton tbtn1 (
        !btn1,
        clk,
        paused
    );
    debounce dbtn2 (
        !btn2,
        clk,
        reset
    );

    always @(posedge clk) begin
        clockCounter <= clockCounter + 1;
        if (clockCounter == WAIT_TIME) begin
            clockCounter <= 0;
            ledCounter   <= ledCounter + 1;
        end

        if (paused) begin
            clockCounter <= 0;
        end

        if (reset) begin
            ledCounter <= 0;
        end
    end

    assign led = ledCounter + 1;
endmodule

module togglebutton (
    input p_btn,
    input clk,
    output reg state
);
    reg prev_state = 1'b0;
    debounce db (
        p_btn,
        clk,
        next_state
    );

    always @(posedge clk) begin
        if (next_state & !prev_state) begin
            state <= ~state;
        end

        prev_state <= next_state;
    end
endmodule


module debounce (
    input p_btn,
    input clk,
    output reg state
);
    reg [3:0] shreg = 4'b0000;

    always @(posedge clk) begin
        shreg <= {shreg[2:0], p_btn};
    end

    // all of shreg &
    assign state = shreg == 4'b1111;
endmodule
