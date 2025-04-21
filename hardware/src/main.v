module main (
    input clk,
    input uart_rx,
    output uart_tx,
    output reg [5:0] led,
    input btn1
);
    wire overrideMemControl;
    wire overrideMemRnW;
    wire [15:0] overrideMemAddr;
    wire [15:0] overrideMemDataIn;
    wire [15:0] overrideMemDataOut;
    reg enable = 0;
    reg slowClk;
    wire done;
    wire start;
    wire [15:0] ir;

    mu0 mu0_inst (
        .clk(slowClk & enable),
        .fastClk(clk),
        .overrideMemControl(overrideMemControl),
        .overrideMemRnW(overrideMemRnW),
        .overrideMemAddr(overrideMemAddr),
        .overrideMemDataIn(overrideMemDataIn),
        .overrideMemDataOut(overrideMemDataOut),
        .enable(enable),
        .reset(btn1),
        .done(done),
        .start(start),
        .ir(ir)
    );

    assign led[0] = ~(slowClk);
    assign led[1] = ~enable;
    assign led[2] = ~overrideMemControl;
    assign led[3] = ~ir[14];
    assign led[4] = ~ir[13];
    assign led[5] = ~ir[12];

    uart uart (
        .clk(clk),
        .uart_rx(uart_rx),
        .uart_tx(uart_tx),
        .overrideMemControl(overrideMemControl),
        .overrideMemRnW(overrideMemRnW),
        .overrideMemAddr(overrideMemAddr),
        .overrideMemDataIn(overrideMemDataIn),
        .overrideMemDataOut(overrideMemDataOut),
        .start(start),
        .enable(enable),
    );

    reg [31:0] clkCounter = 0;
    reg oldStart = 0;
    always @(posedge clk) begin
        clkCounter = clkCounter + 1;
        if (done) begin
            enable = 0;
        end
        if (reset) begin
            enable = 0;
        end
        if (start != oldStart) begin
            oldStart = start;
            enable = 1;
        end
        if (clkCounter > 6318000) begin
            clkCounter = 0;
            slowClk = ~slowClk;
        end
    end
endmodule
